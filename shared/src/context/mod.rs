//! context —— 每台驱动都要走的那几步：**上树那条会话**、落门牌、占线。
//!
//! # 它只余会话（`entry` 那一格撤了）
//! 从前这里揣着一枚**入口孔**（`ENTRY_MARK`，"域自己那扇门"的老口径）。今天各台的门面各
//! 不相同——`uart` 落的是**两枚页**（一具架一枚，见 `driver/uart/adapt/desk.rs`）、`rtc`/`router`
//! 各自铸自己那一枚——故这一层只管**会话**（＋两句每条路都要做的小事：`plate` / `line`）。
//! 要落入口孔的台自己拿着那一枚（`main.rs`/`adapt/boot.rs` 里的局部），[`Context::plate`] 收它。
//!
//! # 一并撤掉的三手
//! `publish` / `offer` / `taken`：那是"一手一格"时代 uart 那两半握手（`Forever` 那一半与
//! `POLL` 那两半）的化身；两条路都改成一具架之后，谁也不等对方，故它们没有读者了。

mod context;
mod step;

pub use context::Context;
pub use step::Step;
