# Circuit
对数字电路及其设计进行模拟，用于更好地学习数字电路，提高抽象思维能力

*目前还在开发初期，随时可能有重大改动*

[doc](https://docs.rs/digicir/latest/digicir/)

---

## Idea
* **所有的逻辑部件(LogicComponent)都使用一套统一的接口，统一的输入，统一的输出**  
它可能长这样：
```rust
struct Component {
    inputs: Vec<SignalId>,
    outputs: Vec<SignalId>,
    inner_graph: Vec<LogicComponent>,
}
```
**各种逻辑部件之间可以相互调用，相互连接**  
**可以自定义逻辑部件**  
**引入并发**

## Implement
介绍一些实现上的想法

* `Signal: Option<bool>` 信号有三种状态：None(无信号)，Some(true), Some(false)
* 信号只由输入产生和逻辑部件生成，任何逻辑部件只能对信号进行读取不可改写
* 信号持有逻辑部件的id，可应用于事件驱动模型
* `signal_pool: Vec<Signal>` 信号池。在信号池中每个信号都拥有唯一的id（索引）
* `circuit_graph: Vec<LogicComponent>` 存储逻辑部件
信号池初始化输入信号后，每次新增逻辑部件，逻辑部件从已有信号中选取输入，并产生输出加入信号池。
* `WiredGate`，门级逻辑部件的实现，直接使用抽象代数类型带来的类型多态，暂时不支持完整的自定义模式。

## 运行模型
* 全局同步：每个时钟周期遍历部件并更新，每次运行只需依次遍历circuit_graph，更新signal_pool  
* 事件驱动：只在信号变化时调度相关部件



