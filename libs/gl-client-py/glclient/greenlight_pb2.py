# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: greenlight.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x10greenlight.proto\x12\ngreenlight\"H\n\x11HsmRequestContext\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\x0c\n\x04\x64\x62id\x18\x02 \x01(\x04\x12\x14\n\x0c\x63\x61pabilities\x18\x03 \x01(\x04\"b\n\x0bHsmResponse\x12\x12\n\nrequest_id\x18\x01 \x01(\r\x12\x0b\n\x03raw\x18\x02 \x01(\x0c\x12\x32\n\x0csigner_state\x18\x05 \x03(\x0b\x32\x1c.greenlight.SignerStateEntry\"\xbf\x01\n\nHsmRequest\x12\x12\n\nrequest_id\x18\x01 \x01(\r\x12.\n\x07\x63ontext\x18\x02 \x01(\x0b\x32\x1d.greenlight.HsmRequestContext\x12\x0b\n\x03raw\x18\x03 \x01(\x0c\x12\x32\n\x0csigner_state\x18\x04 \x03(\x0b\x32\x1c.greenlight.SignerStateEntry\x12,\n\x08requests\x18\x05 \x03(\x0b\x32\x1a.greenlight.PendingRequest\"\x07\n\x05\x45mpty\"O\n\x07\x41\x64\x64ress\x12(\n\x04type\x18\x01 \x01(\x0e\x32\x1a.greenlight.NetAddressType\x12\x0c\n\x04\x61\x64\x64r\x18\x02 \x01(\t\x12\x0c\n\x04port\x18\x03 \x01(\r\"\x10\n\x0eGetInfoRequest\"\xb2\x01\n\x0fGetInfoResponse\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\r\n\x05\x61lias\x18\x02 \x01(\t\x12\r\n\x05\x63olor\x18\x03 \x01(\x0c\x12\x11\n\tnum_peers\x18\x04 \x01(\r\x12&\n\taddresses\x18\x05 \x03(\x0b\x32\x13.greenlight.Address\x12\x0f\n\x07version\x18\x06 \x01(\t\x12\x13\n\x0b\x62lockheight\x18\x07 \x01(\r\x12\x0f\n\x07network\x18\x08 \x01(\t\"\r\n\x0bStopRequest\"\x0e\n\x0cStopResponse\"/\n\x0e\x43onnectRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\t\x12\x0c\n\x04\x61\x64\x64r\x18\x02 \x01(\t\"4\n\x0f\x43onnectResponse\x12\x0f\n\x07node_id\x18\x01 \x01(\t\x12\x10\n\x08\x66\x65\x61tures\x18\x02 \x01(\t\"#\n\x10ListPeersRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\t\"\x81\x01\n\x04Htlc\x12\x11\n\tdirection\x18\x01 \x01(\t\x12\n\n\x02id\x18\x02 \x01(\x04\x12\x0e\n\x06\x61mount\x18\x03 \x01(\t\x12\x0e\n\x06\x65xpiry\x18\x04 \x01(\x04\x12\x14\n\x0cpayment_hash\x18\x05 \x01(\t\x12\r\n\x05state\x18\x06 \x01(\t\x12\x15\n\rlocal_trimmed\x18\x07 \x01(\x08\"(\n\x07\x41liases\x12\r\n\x05local\x18\x01 \x01(\t\x12\x0e\n\x06remote\x18\x02 \x01(\t\"\x8f\x03\n\x07\x43hannel\x12\r\n\x05state\x18\x01 \x01(\t\x12\r\n\x05owner\x18\x02 \x01(\t\x12\"\n\x05\x61lias\x18\x12 \x01(\x0b\x32\x13.greenlight.Aliases\x12\x18\n\x10short_channel_id\x18\x03 \x01(\t\x12\x11\n\tdirection\x18\x04 \x01(\r\x12\x12\n\nchannel_id\x18\x05 \x01(\t\x12\x14\n\x0c\x66unding_txid\x18\x06 \x01(\t\x12\x15\n\rclose_to_addr\x18\x07 \x01(\t\x12\x10\n\x08\x63lose_to\x18\x08 \x01(\t\x12\x0f\n\x07private\x18\t \x01(\x08\x12\r\n\x05total\x18\n \x01(\t\x12\x12\n\ndust_limit\x18\x0b \x01(\t\x12\x11\n\tspendable\x18\x0c \x01(\t\x12\x12\n\nreceivable\x18\r \x01(\t\x12\x1b\n\x13their_to_self_delay\x18\x0e \x01(\r\x12\x19\n\x11our_to_self_delay\x18\x0f \x01(\r\x12\x0e\n\x06status\x18\x10 \x03(\t\x12\x1f\n\x05htlcs\x18\x11 \x03(\x0b\x32\x10.greenlight.Htlc\"\x86\x01\n\x04Peer\x12\n\n\x02id\x18\x01 \x01(\x0c\x12\x11\n\tconnected\x18\x02 \x01(\x08\x12&\n\taddresses\x18\x03 \x03(\x0b\x32\x13.greenlight.Address\x12\x10\n\x08\x66\x65\x61tures\x18\x04 \x01(\t\x12%\n\x08\x63hannels\x18\x05 \x03(\x0b\x32\x13.greenlight.Channel\"4\n\x11ListPeersResponse\x12\x1f\n\x05peers\x18\x01 \x03(\x0b\x32\x10.greenlight.Peer\"3\n\x11\x44isconnectRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\t\x12\r\n\x05\x66orce\x18\x02 \x01(\x08\"\x14\n\x12\x44isconnectResponse\"B\n\x0eNewAddrRequest\x12\x30\n\x0c\x61\x64\x64ress_type\x18\x01 \x01(\x0e\x32\x1a.greenlight.BtcAddressType\"T\n\x0fNewAddrResponse\x12\x30\n\x0c\x61\x64\x64ress_type\x18\x01 \x01(\x0e\x32\x1a.greenlight.BtcAddressType\x12\x0f\n\x07\x61\x64\x64ress\x18\x02 \x01(\t\"=\n\x10ListFundsRequest\x12)\n\x07minconf\x18\x01 \x01(\x0b\x32\x18.greenlight.Confirmation\"\xc3\x01\n\x0fListFundsOutput\x12$\n\x06output\x18\x01 \x01(\x0b\x32\x14.greenlight.Outpoint\x12\"\n\x06\x61mount\x18\x02 \x01(\x0b\x32\x12.greenlight.Amount\x12\x0f\n\x07\x61\x64\x64ress\x18\x04 \x01(\t\x12(\n\x06status\x18\x05 \x01(\x0e\x32\x18.greenlight.OutputStatus\x12\x10\n\x08reserved\x18\x06 \x01(\x08\x12\x19\n\x11reserved_to_block\x18\x07 \x01(\r\"\xac\x01\n\x10ListFundsChannel\x12\x0f\n\x07peer_id\x18\x01 \x01(\x0c\x12\x11\n\tconnected\x18\x02 \x01(\x08\x12\x18\n\x10short_channel_id\x18\x03 \x01(\x04\x12\x17\n\x0four_amount_msat\x18\x04 \x01(\x04\x12\x13\n\x0b\x61mount_msat\x18\x05 \x01(\x04\x12\x14\n\x0c\x66unding_txid\x18\x06 \x01(\x0c\x12\x16\n\x0e\x66unding_output\x18\x07 \x01(\r\"q\n\x11ListFundsResponse\x12,\n\x07outputs\x18\x01 \x03(\x0b\x32\x1b.greenlight.ListFundsOutput\x12.\n\x08\x63hannels\x18\x02 \x03(\x0b\x32\x1c.greenlight.ListFundsChannel\"a\n\x07\x46\x65\x65rate\x12+\n\x06preset\x18\x01 \x01(\x0e\x32\x19.greenlight.FeeratePresetH\x00\x12\x0f\n\x05perkw\x18\x05 \x01(\x04H\x00\x12\x0f\n\x05perkb\x18\x06 \x01(\x04H\x00\x42\x07\n\x05value\"\x1e\n\x0c\x43onfirmation\x12\x0e\n\x06\x62locks\x18\x01 \x01(\r\"\xc0\x01\n\x0fWithdrawRequest\x12\x13\n\x0b\x64\x65stination\x18\x01 \x01(\t\x12\"\n\x06\x61mount\x18\x02 \x01(\x0b\x32\x12.greenlight.Amount\x12$\n\x07\x66\x65\x65rate\x18\x03 \x01(\x0b\x32\x13.greenlight.Feerate\x12)\n\x07minconf\x18\x07 \x01(\x0b\x32\x18.greenlight.Confirmation\x12#\n\x05utxos\x18\x08 \x03(\x0b\x32\x14.greenlight.Outpoint\",\n\x10WithdrawResponse\x12\n\n\x02tx\x18\x01 \x01(\x0c\x12\x0c\n\x04txid\x18\x02 \x01(\x0c\"\xbe\x01\n\x12\x46undChannelRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\"\n\x06\x61mount\x18\x02 \x01(\x0b\x32\x12.greenlight.Amount\x12$\n\x07\x66\x65\x65rate\x18\x03 \x01(\x0b\x32\x13.greenlight.Feerate\x12\x10\n\x08\x61nnounce\x18\x07 \x01(\x08\x12)\n\x07minconf\x18\x08 \x01(\x0b\x32\x18.greenlight.Confirmation\x12\x10\n\x08\x63lose_to\x18\n \x01(\t\"(\n\x08Outpoint\x12\x0c\n\x04txid\x18\x01 \x01(\x0c\x12\x0e\n\x06outnum\x18\x02 \x01(\r\"o\n\x13\x46undChannelResponse\x12\n\n\x02tx\x18\x01 \x01(\x0c\x12&\n\x08outpoint\x18\x02 \x01(\x0b\x32\x14.greenlight.Outpoint\x12\x12\n\nchannel_id\x18\x03 \x01(\x0c\x12\x10\n\x08\x63lose_to\x18\x04 \x01(\t\"\x1a\n\x07Timeout\x12\x0f\n\x07seconds\x18\x01 \x01(\r\"!\n\x0e\x42itcoinAddress\x12\x0f\n\x07\x61\x64\x64ress\x18\x01 \x01(\t\"\x87\x01\n\x13\x43loseChannelRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12.\n\x11unilateraltimeout\x18\x02 \x01(\x0b\x32\x13.greenlight.Timeout\x12/\n\x0b\x64\x65stination\x18\x03 \x01(\x0b\x32\x1a.greenlight.BitcoinAddress\"b\n\x14\x43loseChannelResponse\x12\x30\n\nclose_type\x18\x01 \x01(\x0e\x32\x1c.greenlight.CloseChannelType\x12\n\n\x02tx\x18\x02 \x01(\x0c\x12\x0c\n\x04txid\x18\x03 \x01(\x0c\"l\n\x06\x41mount\x12\x16\n\x0cmillisatoshi\x18\x01 \x01(\x04H\x00\x12\x11\n\x07satoshi\x18\x02 \x01(\x04H\x00\x12\x11\n\x07\x62itcoin\x18\x03 \x01(\x04H\x00\x12\r\n\x03\x61ll\x18\x04 \x01(\x08H\x00\x12\r\n\x03\x61ny\x18\x05 \x01(\x08H\x00\x42\x06\n\x04unit\"j\n\x0eInvoiceRequest\x12\"\n\x06\x61mount\x18\x01 \x01(\x0b\x32\x12.greenlight.Amount\x12\r\n\x05label\x18\x02 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x03 \x01(\t\x12\x10\n\x08preimage\x18\x04 \x01(\x0c\"\x8d\x02\n\x07Invoice\x12\r\n\x05label\x18\x01 \x01(\t\x12\x13\n\x0b\x64\x65scription\x18\x02 \x01(\t\x12\"\n\x06\x61mount\x18\x03 \x01(\x0b\x32\x12.greenlight.Amount\x12$\n\x08received\x18\x04 \x01(\x0b\x32\x12.greenlight.Amount\x12)\n\x06status\x18\x05 \x01(\x0e\x32\x19.greenlight.InvoiceStatus\x12\x14\n\x0cpayment_time\x18\x06 \x01(\r\x12\x13\n\x0b\x65xpiry_time\x18\x07 \x01(\r\x12\x0e\n\x06\x62olt11\x18\x08 \x01(\t\x12\x14\n\x0cpayment_hash\x18\t \x01(\x0c\x12\x18\n\x10payment_preimage\x18\n \x01(\x0c\"\x8c\x01\n\nPayRequest\x12\x0e\n\x06\x62olt11\x18\x01 \x01(\t\x12\"\n\x06\x61mount\x18\x02 \x01(\x0b\x32\x12.greenlight.Amount\x12\x0f\n\x07timeout\x18\x03 \x01(\r\x12\x15\n\rmaxfeepercent\x18\x04 \x01(\x01\x12\"\n\x06maxfee\x18\x05 \x01(\x0b\x32\x12.greenlight.Amount\"\xfc\x01\n\x07Payment\x12\x13\n\x0b\x64\x65stination\x18\x01 \x01(\x0c\x12\x14\n\x0cpayment_hash\x18\x02 \x01(\x0c\x12\x18\n\x10payment_preimage\x18\x03 \x01(\x0c\x12%\n\x06status\x18\x04 \x01(\x0e\x32\x15.greenlight.PayStatus\x12\"\n\x06\x61mount\x18\x05 \x01(\x0b\x32\x12.greenlight.Amount\x12\'\n\x0b\x61mount_sent\x18\x06 \x01(\x0b\x32\x12.greenlight.Amount\x12\x0e\n\x06\x62olt11\x18\x07 \x01(\t\x12\x12\n\ncreated_at\x18\x08 \x01(\x01\x12\x14\n\x0c\x63ompleted_at\x18\t \x01(\x04\"C\n\x11PaymentIdentifier\x12\x10\n\x06\x62olt11\x18\x01 \x01(\tH\x00\x12\x16\n\x0cpayment_hash\x18\x02 \x01(\x0cH\x00\x42\x04\n\x02id\"H\n\x13ListPaymentsRequest\x12\x31\n\nidentifier\x18\x01 \x01(\x0b\x32\x1d.greenlight.PaymentIdentifier\"=\n\x14ListPaymentsResponse\x12%\n\x08payments\x18\x01 \x03(\x0b\x32\x13.greenlight.Payment\"W\n\x11InvoiceIdentifier\x12\x0f\n\x05label\x18\x01 \x01(\tH\x00\x12\x13\n\tinvstring\x18\x02 \x01(\tH\x00\x12\x16\n\x0cpayment_hash\x18\x03 \x01(\x0cH\x00\x42\x04\n\x02id\"H\n\x13ListInvoicesRequest\x12\x31\n\nidentifier\x18\x01 \x01(\x0b\x32\x1d.greenlight.InvoiceIdentifier\"\x16\n\x14StreamIncomingFilter\"=\n\x14ListInvoicesResponse\x12%\n\x08invoices\x18\x01 \x03(\x0b\x32\x13.greenlight.Invoice\"\'\n\x08TlvField\x12\x0c\n\x04type\x18\x01 \x01(\x04\x12\r\n\x05value\x18\x02 \x01(\x0c\"\xa5\x01\n\x0fOffChainPayment\x12\r\n\x05label\x18\x01 \x01(\t\x12\x10\n\x08preimage\x18\x02 \x01(\x0c\x12\"\n\x06\x61mount\x18\x03 \x01(\x0b\x32\x12.greenlight.Amount\x12\'\n\textratlvs\x18\x04 \x03(\x0b\x32\x14.greenlight.TlvField\x12\x14\n\x0cpayment_hash\x18\x05 \x01(\x0c\x12\x0e\n\x06\x62olt11\x18\x06 \x01(\t\"M\n\x0fIncomingPayment\x12/\n\x08offchain\x18\x01 \x01(\x0b\x32\x1b.greenlight.OffChainPaymentH\x00\x42\t\n\x07\x64\x65tails\"x\n\x0cRoutehintHop\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\x18\n\x10short_channel_id\x18\x02 \x01(\t\x12\x10\n\x08\x66\x65\x65_base\x18\x03 \x01(\x04\x12\x10\n\x08\x66\x65\x65_prop\x18\x04 \x01(\r\x12\x19\n\x11\x63ltv_expiry_delta\x18\x05 \x01(\r\"3\n\tRoutehint\x12&\n\x04hops\x18\x01 \x03(\x0b\x32\x18.greenlight.RoutehintHop\"\xa8\x01\n\x0eKeysendRequest\x12\x0f\n\x07node_id\x18\x01 \x01(\x0c\x12\"\n\x06\x61mount\x18\x02 \x01(\x0b\x32\x12.greenlight.Amount\x12\r\n\x05label\x18\x03 \x01(\t\x12)\n\nroutehints\x18\x04 \x03(\x0b\x32\x15.greenlight.Routehint\x12\'\n\textratlvs\x18\x05 \x03(\x0b\x32\x14.greenlight.TlvField\"\x12\n\x10StreamLogRequest\"\x18\n\x08LogEntry\x12\x0c\n\x04line\x18\x01 \x01(\t\"?\n\x10SignerStateEntry\x12\x0f\n\x07version\x18\x01 \x01(\x04\x12\x0b\n\x03key\x18\x02 \x01(\t\x12\r\n\x05value\x18\x03 \x01(\x0c\"d\n\x0ePendingRequest\x12\x0f\n\x07request\x18\x01 \x01(\x0c\x12\x0b\n\x03uri\x18\x02 \x01(\t\x12\x11\n\tsignature\x18\x03 \x01(\x0c\x12\x0e\n\x06pubkey\x18\x04 \x01(\x0c\x12\x11\n\ttimestamp\x18\x05 \x01(\x04\"=\n\nNodeConfig\x12/\n\x0bstartupmsgs\x18\x01 \x03(\x0b\x32\x1a.greenlight.StartupMessage\"!\n\x08GlConfig\x12\x15\n\rclose_to_addr\x18\x01 \x01(\t\"3\n\x0eStartupMessage\x12\x0f\n\x07request\x18\x01 \x01(\x0c\x12\x10\n\x08response\x18\x02 \x01(\x0c\"\x18\n\x16StreamCustommsgRequest\"-\n\tCustommsg\x12\x0f\n\x07peer_id\x18\x01 \x01(\x0c\x12\x0f\n\x07payload\x18\x02 \x01(\x0c*:\n\x0eNetAddressType\x12\x08\n\x04Ipv4\x10\x00\x12\x08\n\x04Ipv6\x10\x01\x12\t\n\x05TorV2\x10\x02\x12\t\n\x05TorV3\x10\x03*-\n\x0e\x42tcAddressType\x12\n\n\x06\x42\x45\x43H32\x10\x00\x12\x0f\n\x0bP2SH_SEGWIT\x10\x01*.\n\x0cOutputStatus\x12\r\n\tCONFIRMED\x10\x00\x12\x0f\n\x0bUNCONFIRMED\x10\x01*1\n\rFeeratePreset\x12\n\n\x06NORMAL\x10\x00\x12\x08\n\x04SLOW\x10\x01\x12\n\n\x06URGENT\x10\x02*.\n\x10\x43loseChannelType\x12\n\n\x06MUTUAL\x10\x00\x12\x0e\n\nUNILATERAL\x10\x01*2\n\rInvoiceStatus\x12\n\n\x06UNPAID\x10\x00\x12\x08\n\x04PAID\x10\x01\x12\x0b\n\x07\x45XPIRED\x10\x02*2\n\tPayStatus\x12\x0b\n\x07PENDING\x10\x00\x12\x0c\n\x08\x43OMPLETE\x10\x01\x12\n\n\x06\x46\x41ILED\x10\x02\x32\xa4\x0c\n\x04Node\x12G\n\x07GetInfo\x12\x1a.greenlight.GetInfoRequest\x1a\x1b.greenlight.GetInfoResponse\"\x03\x88\x02\x01\x12>\n\x04Stop\x12\x17.greenlight.StopRequest\x1a\x18.greenlight.StopResponse\"\x03\x88\x02\x01\x12K\n\x0b\x43onnectPeer\x12\x1a.greenlight.ConnectRequest\x1a\x1b.greenlight.ConnectResponse\"\x03\x88\x02\x01\x12M\n\tListPeers\x12\x1c.greenlight.ListPeersRequest\x1a\x1d.greenlight.ListPeersResponse\"\x03\x88\x02\x01\x12P\n\nDisconnect\x12\x1d.greenlight.DisconnectRequest\x1a\x1e.greenlight.DisconnectResponse\"\x03\x88\x02\x01\x12G\n\x07NewAddr\x12\x1a.greenlight.NewAddrRequest\x1a\x1b.greenlight.NewAddrResponse\"\x03\x88\x02\x01\x12M\n\tListFunds\x12\x1c.greenlight.ListFundsRequest\x1a\x1d.greenlight.ListFundsResponse\"\x03\x88\x02\x01\x12J\n\x08Withdraw\x12\x1b.greenlight.WithdrawRequest\x1a\x1c.greenlight.WithdrawResponse\"\x03\x88\x02\x01\x12S\n\x0b\x46undChannel\x12\x1e.greenlight.FundChannelRequest\x1a\x1f.greenlight.FundChannelResponse\"\x03\x88\x02\x01\x12V\n\x0c\x43loseChannel\x12\x1f.greenlight.CloseChannelRequest\x1a .greenlight.CloseChannelResponse\"\x03\x88\x02\x01\x12\x45\n\rCreateInvoice\x12\x1a.greenlight.InvoiceRequest\x1a\x13.greenlight.Invoice\"\x03\x88\x02\x01\x12\x37\n\x03Pay\x12\x16.greenlight.PayRequest\x1a\x13.greenlight.Payment\"\x03\x88\x02\x01\x12?\n\x07Keysend\x12\x1a.greenlight.KeysendRequest\x1a\x13.greenlight.Payment\"\x03\x88\x02\x01\x12S\n\x0cListPayments\x12\x1f.greenlight.ListPaymentsRequest\x1a .greenlight.ListPaymentsResponse\"\x00\x12S\n\x0cListInvoices\x12\x1f.greenlight.ListInvoicesRequest\x1a .greenlight.ListInvoicesResponse\"\x00\x12S\n\x0eStreamIncoming\x12 .greenlight.StreamIncomingFilter\x1a\x1b.greenlight.IncomingPayment\"\x00\x30\x01\x12\x43\n\tStreamLog\x12\x1c.greenlight.StreamLogRequest\x1a\x14.greenlight.LogEntry\"\x00\x30\x01\x12P\n\x0fStreamCustommsg\x12\".greenlight.StreamCustommsgRequest\x1a\x15.greenlight.Custommsg\"\x00\x30\x01\x12\x42\n\x11StreamHsmRequests\x12\x11.greenlight.Empty\x1a\x16.greenlight.HsmRequest\"\x00\x30\x01\x12\x41\n\x11RespondHsmRequest\x12\x17.greenlight.HsmResponse\x1a\x11.greenlight.Empty\"\x00\x12\x36\n\tConfigure\x12\x14.greenlight.GlConfig\x1a\x11.greenlight.Empty\"\x00\x32s\n\x03Hsm\x12<\n\x07Request\x12\x16.greenlight.HsmRequest\x1a\x17.greenlight.HsmResponse\"\x00\x12.\n\x04Ping\x12\x11.greenlight.Empty\x1a\x11.greenlight.Empty\"\x00\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'greenlight_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _NODE.methods_by_name['GetInfo']._options = None
  _NODE.methods_by_name['GetInfo']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['Stop']._options = None
  _NODE.methods_by_name['Stop']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['ConnectPeer']._options = None
  _NODE.methods_by_name['ConnectPeer']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['ListPeers']._options = None
  _NODE.methods_by_name['ListPeers']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['Disconnect']._options = None
  _NODE.methods_by_name['Disconnect']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['NewAddr']._options = None
  _NODE.methods_by_name['NewAddr']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['ListFunds']._options = None
  _NODE.methods_by_name['ListFunds']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['Withdraw']._options = None
  _NODE.methods_by_name['Withdraw']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['FundChannel']._options = None
  _NODE.methods_by_name['FundChannel']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['CloseChannel']._options = None
  _NODE.methods_by_name['CloseChannel']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['CreateInvoice']._options = None
  _NODE.methods_by_name['CreateInvoice']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['Pay']._options = None
  _NODE.methods_by_name['Pay']._serialized_options = b'\210\002\001'
  _NODE.methods_by_name['Keysend']._options = None
  _NODE.methods_by_name['Keysend']._serialized_options = b'\210\002\001'
  _globals['_NETADDRESSTYPE']._serialized_start=5843
  _globals['_NETADDRESSTYPE']._serialized_end=5901
  _globals['_BTCADDRESSTYPE']._serialized_start=5903
  _globals['_BTCADDRESSTYPE']._serialized_end=5948
  _globals['_OUTPUTSTATUS']._serialized_start=5950
  _globals['_OUTPUTSTATUS']._serialized_end=5996
  _globals['_FEERATEPRESET']._serialized_start=5998
  _globals['_FEERATEPRESET']._serialized_end=6047
  _globals['_CLOSECHANNELTYPE']._serialized_start=6049
  _globals['_CLOSECHANNELTYPE']._serialized_end=6095
  _globals['_INVOICESTATUS']._serialized_start=6097
  _globals['_INVOICESTATUS']._serialized_end=6147
  _globals['_PAYSTATUS']._serialized_start=6149
  _globals['_PAYSTATUS']._serialized_end=6199
  _globals['_HSMREQUESTCONTEXT']._serialized_start=32
  _globals['_HSMREQUESTCONTEXT']._serialized_end=104
  _globals['_HSMRESPONSE']._serialized_start=106
  _globals['_HSMRESPONSE']._serialized_end=204
  _globals['_HSMREQUEST']._serialized_start=207
  _globals['_HSMREQUEST']._serialized_end=398
  _globals['_EMPTY']._serialized_start=400
  _globals['_EMPTY']._serialized_end=407
  _globals['_ADDRESS']._serialized_start=409
  _globals['_ADDRESS']._serialized_end=488
  _globals['_GETINFOREQUEST']._serialized_start=490
  _globals['_GETINFOREQUEST']._serialized_end=506
  _globals['_GETINFORESPONSE']._serialized_start=509
  _globals['_GETINFORESPONSE']._serialized_end=687
  _globals['_STOPREQUEST']._serialized_start=689
  _globals['_STOPREQUEST']._serialized_end=702
  _globals['_STOPRESPONSE']._serialized_start=704
  _globals['_STOPRESPONSE']._serialized_end=718
  _globals['_CONNECTREQUEST']._serialized_start=720
  _globals['_CONNECTREQUEST']._serialized_end=767
  _globals['_CONNECTRESPONSE']._serialized_start=769
  _globals['_CONNECTRESPONSE']._serialized_end=821
  _globals['_LISTPEERSREQUEST']._serialized_start=823
  _globals['_LISTPEERSREQUEST']._serialized_end=858
  _globals['_HTLC']._serialized_start=861
  _globals['_HTLC']._serialized_end=990
  _globals['_ALIASES']._serialized_start=992
  _globals['_ALIASES']._serialized_end=1032
  _globals['_CHANNEL']._serialized_start=1035
  _globals['_CHANNEL']._serialized_end=1434
  _globals['_PEER']._serialized_start=1437
  _globals['_PEER']._serialized_end=1571
  _globals['_LISTPEERSRESPONSE']._serialized_start=1573
  _globals['_LISTPEERSRESPONSE']._serialized_end=1625
  _globals['_DISCONNECTREQUEST']._serialized_start=1627
  _globals['_DISCONNECTREQUEST']._serialized_end=1678
  _globals['_DISCONNECTRESPONSE']._serialized_start=1680
  _globals['_DISCONNECTRESPONSE']._serialized_end=1700
  _globals['_NEWADDRREQUEST']._serialized_start=1702
  _globals['_NEWADDRREQUEST']._serialized_end=1768
  _globals['_NEWADDRRESPONSE']._serialized_start=1770
  _globals['_NEWADDRRESPONSE']._serialized_end=1854
  _globals['_LISTFUNDSREQUEST']._serialized_start=1856
  _globals['_LISTFUNDSREQUEST']._serialized_end=1917
  _globals['_LISTFUNDSOUTPUT']._serialized_start=1920
  _globals['_LISTFUNDSOUTPUT']._serialized_end=2115
  _globals['_LISTFUNDSCHANNEL']._serialized_start=2118
  _globals['_LISTFUNDSCHANNEL']._serialized_end=2290
  _globals['_LISTFUNDSRESPONSE']._serialized_start=2292
  _globals['_LISTFUNDSRESPONSE']._serialized_end=2405
  _globals['_FEERATE']._serialized_start=2407
  _globals['_FEERATE']._serialized_end=2504
  _globals['_CONFIRMATION']._serialized_start=2506
  _globals['_CONFIRMATION']._serialized_end=2536
  _globals['_WITHDRAWREQUEST']._serialized_start=2539
  _globals['_WITHDRAWREQUEST']._serialized_end=2731
  _globals['_WITHDRAWRESPONSE']._serialized_start=2733
  _globals['_WITHDRAWRESPONSE']._serialized_end=2777
  _globals['_FUNDCHANNELREQUEST']._serialized_start=2780
  _globals['_FUNDCHANNELREQUEST']._serialized_end=2970
  _globals['_OUTPOINT']._serialized_start=2972
  _globals['_OUTPOINT']._serialized_end=3012
  _globals['_FUNDCHANNELRESPONSE']._serialized_start=3014
  _globals['_FUNDCHANNELRESPONSE']._serialized_end=3125
  _globals['_TIMEOUT']._serialized_start=3127
  _globals['_TIMEOUT']._serialized_end=3153
  _globals['_BITCOINADDRESS']._serialized_start=3155
  _globals['_BITCOINADDRESS']._serialized_end=3188
  _globals['_CLOSECHANNELREQUEST']._serialized_start=3191
  _globals['_CLOSECHANNELREQUEST']._serialized_end=3326
  _globals['_CLOSECHANNELRESPONSE']._serialized_start=3328
  _globals['_CLOSECHANNELRESPONSE']._serialized_end=3426
  _globals['_AMOUNT']._serialized_start=3428
  _globals['_AMOUNT']._serialized_end=3536
  _globals['_INVOICEREQUEST']._serialized_start=3538
  _globals['_INVOICEREQUEST']._serialized_end=3644
  _globals['_INVOICE']._serialized_start=3647
  _globals['_INVOICE']._serialized_end=3916
  _globals['_PAYREQUEST']._serialized_start=3919
  _globals['_PAYREQUEST']._serialized_end=4059
  _globals['_PAYMENT']._serialized_start=4062
  _globals['_PAYMENT']._serialized_end=4314
  _globals['_PAYMENTIDENTIFIER']._serialized_start=4316
  _globals['_PAYMENTIDENTIFIER']._serialized_end=4383
  _globals['_LISTPAYMENTSREQUEST']._serialized_start=4385
  _globals['_LISTPAYMENTSREQUEST']._serialized_end=4457
  _globals['_LISTPAYMENTSRESPONSE']._serialized_start=4459
  _globals['_LISTPAYMENTSRESPONSE']._serialized_end=4520
  _globals['_INVOICEIDENTIFIER']._serialized_start=4522
  _globals['_INVOICEIDENTIFIER']._serialized_end=4609
  _globals['_LISTINVOICESREQUEST']._serialized_start=4611
  _globals['_LISTINVOICESREQUEST']._serialized_end=4683
  _globals['_STREAMINCOMINGFILTER']._serialized_start=4685
  _globals['_STREAMINCOMINGFILTER']._serialized_end=4707
  _globals['_LISTINVOICESRESPONSE']._serialized_start=4709
  _globals['_LISTINVOICESRESPONSE']._serialized_end=4770
  _globals['_TLVFIELD']._serialized_start=4772
  _globals['_TLVFIELD']._serialized_end=4811
  _globals['_OFFCHAINPAYMENT']._serialized_start=4814
  _globals['_OFFCHAINPAYMENT']._serialized_end=4979
  _globals['_INCOMINGPAYMENT']._serialized_start=4981
  _globals['_INCOMINGPAYMENT']._serialized_end=5058
  _globals['_ROUTEHINTHOP']._serialized_start=5060
  _globals['_ROUTEHINTHOP']._serialized_end=5180
  _globals['_ROUTEHINT']._serialized_start=5182
  _globals['_ROUTEHINT']._serialized_end=5233
  _globals['_KEYSENDREQUEST']._serialized_start=5236
  _globals['_KEYSENDREQUEST']._serialized_end=5404
  _globals['_STREAMLOGREQUEST']._serialized_start=5406
  _globals['_STREAMLOGREQUEST']._serialized_end=5424
  _globals['_LOGENTRY']._serialized_start=5426
  _globals['_LOGENTRY']._serialized_end=5450
  _globals['_SIGNERSTATEENTRY']._serialized_start=5452
  _globals['_SIGNERSTATEENTRY']._serialized_end=5515
  _globals['_PENDINGREQUEST']._serialized_start=5517
  _globals['_PENDINGREQUEST']._serialized_end=5617
  _globals['_NODECONFIG']._serialized_start=5619
  _globals['_NODECONFIG']._serialized_end=5680
  _globals['_GLCONFIG']._serialized_start=5682
  _globals['_GLCONFIG']._serialized_end=5715
  _globals['_STARTUPMESSAGE']._serialized_start=5717
  _globals['_STARTUPMESSAGE']._serialized_end=5768
  _globals['_STREAMCUSTOMMSGREQUEST']._serialized_start=5770
  _globals['_STREAMCUSTOMMSGREQUEST']._serialized_end=5794
  _globals['_CUSTOMMSG']._serialized_start=5796
  _globals['_CUSTOMMSG']._serialized_end=5841
  _globals['_NODE']._serialized_start=6202
  _globals['_NODE']._serialized_end=7774
  _globals['_HSM']._serialized_start=7776
  _globals['_HSM']._serialized_end=7891
# @@protoc_insertion_point(module_scope)
