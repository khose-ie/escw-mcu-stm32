pub enum UartEvent
{
    TxHalf,
    TxCompleted,
    TxAborted,
    RxHalf,
    RxCompleted(u16),
    RxAborted,
    TxRxAborted,
    Error,
}
