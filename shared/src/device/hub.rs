use alloc::string::ToString;

use env::Reason as Died;
use env::{PieToken, Wait};
use hub_api::Deed;
use hub_client as hub;
use system_api::operator::path::Path;
use system_client::operator::Face;

use crate::fail::Fail;

use super::Ask;

/// 只能问主人自己交来的那一枚（内核那一问 `Join` 只许同队或父域，而 hub 与驱动是兄弟）
/// **它不持树那条会话**：树上那几手每次按调用方给的 Face 走（会话归 Context
/// 四个客人各持各的）
/// crate::driver::shared::context::Context
pub struct Hub {
    bond: hub::Face,
    list: hub::Face,
    sensor: PieToken,
}

impl Hub {
    /// 两枚面各找一趟（译号带重试 ＋ 取那一枚，落在 Face::tile 上）：hub 可能落得比
    pub fn find(tree: &Face, died: Died, ms: Wait) -> Result<Hub, Fail> {
        // **路是 `/svc/hub/<面>`**（容器那一段接 `hub` 那一段，末段是那一枚面）——**不是
        // `/svc/drv/...`**：hub 是**服务那一层**里的一位（与驱动平级），故头一段是 `SVC`
        // 而不是驱动那一家两段。
        let service = tree
            .resolve(system_api::namespace::SERVICE, ms)
            .map_err(|_| Fail::at(died, "name"))?;
        let hub_road = hub::SERVICE_PATH;
        let bond = face_of(
            tree,
            service,
            &hub_road
                .try_join(hub::Grant::Bond.name())
                .ok_or(Fail::at(died, "name"))?,
            died,
            ms,
        )?;
        let list = face_of(
            tree,
            service,
            &hub_road
                .try_join(hub::Grant::List.name())
                .ok_or(Fail::at(died, "name"))?,
            died,
            ms,
        )?;
        let sensor = env::hole::unseal(hub::ALIVE_MARK, env::HoleLimits::default())
            .map_err(|_| Fail::at(died, "hub"))?;
        Ok(Hub { bond, list, sensor })
    }

    /// **认领一台**：报名（Hub::bond，幂等）→ 列册（取名字）→ 树上找那一格 → 认领 ⇒ 一张契
    /// **"哪一台"由"你找的是哪一格"定**（见 hub_api）：`ask.name = None`
    pub fn claim(&self, tree: &Face, ask: &Ask, died: Died, ms: Wait) -> Result<Deed, Fail> {
        let class = ask.class.to_string();
        self.bond
            .bond(class.clone(), ms)
            .map_err(|_| Fail::at(died, "bond"))?;
        let name = match ask.name {
            Some(want) => want.to_string(),
            None => {
                let window = self
                    .list
                    .list(class.clone(), 0, ms)
                    .map_err(|_| Fail::at(died, "list"))?;
                // 那一台，下一步相同）。
                window.name(0).ok_or(Fail::at(died, "list"))?.clone()
            }
        };
        // 设备那一轴是 `/dev/<类>/<名>`（**顶层那一层**——与 `/svc` 平级，见 hub::DEV）。
        // 轴那一段是常量（`DEV_ROAD`），类与名来自报文 ⇒ 走运行期那一手（答 `None` 就报步名）。
        let devices = tree
            .resolve(system_api::namespace::DEVICE, ms)
            .map_err(|_| Fail::at(died, "tree"))?;
        let road = system_api::operator::PathBuf::try_new(&class)
            .and_then(|path| path.as_path().try_join(&name))
            .ok_or(Fail::at(died, "name"))?;
        let door = tree
            .tile_from(devices, &road, ms)
            .and_then(|tile| tile.token(ms))
            .map_err(|_| Fail::at(died, "tree"))?;
        // **认领打的是那一格上挂着的那一枚**（不是 hub 的门面）：hub 据"哪一枚孔响了"认台。
        let Ok(face) = hub::Face::of(door) else {
            return Err(Fail::at(died, "claim"));
        };
        face.claim(ask.kind, ask.access, ask.policy, self.sensor, ms)
            .map_err(|_| Fail::at(died, "claim"))
    }
}

/// 树上找一枚门牌（**沿一条路，答那一枚**）——三个客人（本手两处 ＋ 各域自己那几处）共用
/// **它不吞错**：失败一律折 `Fail::at(died, "hub")`（"设备账那两枚面没找着"），由调用方给号
fn face_of(
    tree: &Face,
    base: system_api::operator::Entry,
    road: &Path,
    died: Died,
    ms: Wait,
) -> Result<hub::Face, Fail> {
    let door = tree
        .tile_from(base, road, ms)
        .and_then(|entry| entry.token(ms))
        .map_err(|_| Fail::at(died, "hub"))?;
    hub::Face::of(door).map_err(|_| Fail::at(died, "hub"))
}
