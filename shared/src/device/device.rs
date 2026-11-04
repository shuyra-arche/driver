use ::resource::dock::{Dock, View};
use env::PieToken;

/// **一台设备的持有者手里那一样**：那一页的映射（域活多久它活多久）
pub struct Device {
    dock: Dock,
}

impl Device {
    /// 契里那一枚门闩 → 一页映射
    /// **失败那一格由调用方命名**（`"device open failed"` / `"docks"`——**步名**
    pub fn open(page: PieToken) -> Result<Device, ()> {
        let dock = Dock::open(page).map_err(|_| ())?;
        Ok(Device { dock })
    }

    /// 那一页的视图（View 是 `Copy`：常驻那一圈每醒一次取一份）
    pub fn view(&self) -> View {
        self.dock.view()
    }
}
