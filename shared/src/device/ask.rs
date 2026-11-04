use env::{Access, PieKind, Policy};

/// 它是**声明**（`const` 可造：三格全是字面量 / 枚举），各驱动写在**自己那一域**里——装配表
#[derive(Clone, Copy)]
pub struct Ask {
    /// 树里认的类：hub 按它把设备归到 `/dev/<类>` 那一块窗格底下，也按它立那一枚**盟**
    pub class: &'static str,
    /// **点名要哪一台**；`None` = 这一类里**头一台**（区首址最小那台，见 Hub::claim）
    pub name: Option<&'static str>,
    /// 什么种类（`Pole` = 一段内存 / `Nole` = 空载荷的信号）
    pub kind: PieKind,
    /// 要多少权（`FETCH` 读、`FETCH_STORE` 读写——设备寄存器面要读写）
    pub access: Access,
    /// **形态**（`ONLY` = 独占：设备 `reg` 段那一枚内核就是那么发的，一枚门闩只许一个使用者）
    pub policy: Policy,
}
