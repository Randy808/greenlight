// This file was generated by `gengrpc` from the CLN JSON-Schema.
// Do not edit this file.
//

pub mod cln;
pub mod greenlight;

/// Variants prefixed with `Gl` are deprecated and will eventually be removed.
#[derive(Clone, Debug)]
pub enum Request {
    GlGetinfo(greenlight::GetInfoRequest),
    GlStop(greenlight::StopRequest),
    GlListPeers(greenlight::ListPeersRequest),
    GlDisconnect(greenlight::DisconnectRequest),
    GlNewAddr(greenlight::NewAddrRequest),
    GlListFunds(greenlight::ListFundsRequest),
    GlWithdraw(greenlight::WithdrawRequest),
    GlFundChannel(greenlight::FundChannelRequest),
    GlCloseChannel(greenlight::CloseChannelRequest),
    GlCreateInvoice(greenlight::InvoiceRequest),
    GlPay(greenlight::PayRequest),
    GlKeysend(greenlight::KeysendRequest),
    GlListPayments(greenlight::ListPaymentsRequest),
    GlListInvoices(greenlight::ListInvoicesRequest),
    GlConnectPeer(greenlight::ConnectRequest),
    GlConfig(greenlight::GlConfig),
    Getinfo(cln::GetinfoRequest),
    ListPeers(cln::ListpeersRequest),
    ListFunds(cln::ListfundsRequest),
    SendPay(cln::SendpayRequest),
    ListChannels(cln::ListchannelsRequest),
    AddGossip(cln::AddgossipRequest),
    AutoCleanInvoice(cln::AutocleaninvoiceRequest),
    CheckMessage(cln::CheckmessageRequest),
    Close(cln::CloseRequest),
    Connect(cln::ConnectRequest),
    CreateInvoice(cln::CreateinvoiceRequest),
    Datastore(cln::DatastoreRequest),
    CreateOnion(cln::CreateonionRequest),
    DelDatastore(cln::DeldatastoreRequest),
    DelExpiredInvoice(cln::DelexpiredinvoiceRequest),
    DelInvoice(cln::DelinvoiceRequest),
    Invoice(cln::InvoiceRequest),
    ListDatastore(cln::ListdatastoreRequest),
    ListInvoices(cln::ListinvoicesRequest),
    SendOnion(cln::SendonionRequest),
    ListSendPays(cln::ListsendpaysRequest),
    ListTransactions(cln::ListtransactionsRequest),
    Pay(cln::PayRequest),
    ListNodes(cln::ListnodesRequest),
    WaitAnyInvoice(cln::WaitanyinvoiceRequest),
    WaitInvoice(cln::WaitinvoiceRequest),
    WaitSendPay(cln::WaitsendpayRequest),
    NewAddr(cln::NewaddrRequest),
    Withdraw(cln::WithdrawRequest),
    KeySend(cln::KeysendRequest),
    FundPsbt(cln::FundpsbtRequest),
    SendPsbt(cln::SendpsbtRequest),
    SignPsbt(cln::SignpsbtRequest),
    UtxoPsbt(cln::UtxopsbtRequest),
    TxDiscard(cln::TxdiscardRequest),
    TxPrepare(cln::TxprepareRequest),
    TxSend(cln::TxsendRequest),
    Disconnect(cln::DisconnectRequest),
    Feerates(cln::FeeratesRequest),
    FundChannel(cln::FundchannelRequest),
    GetRoute(cln::GetrouteRequest),
    ListForwards(cln::ListforwardsRequest),
    ListPays(cln::ListpaysRequest),
    Ping(cln::PingRequest),
    SetChannel(cln::SetchannelRequest),
    SignMessage(cln::SignmessageRequest),
    Stop(cln::StopRequest),
    ListClosedChannels(cln::ListclosedchannelsRequest),
    StaticBackup(cln::StaticbackupRequest),
}
