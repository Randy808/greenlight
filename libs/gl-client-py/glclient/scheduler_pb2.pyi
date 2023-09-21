"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""
import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import greenlight_pb2
import sys
import typing

if sys.version_info >= (3, 10):
    import typing as typing_extensions
else:
    import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

class _ChallengeScope:
    ValueType = typing.NewType("ValueType", builtins.int)
    V: typing_extensions.TypeAlias = ValueType

class _ChallengeScopeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_ChallengeScope.ValueType], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor
    REGISTER: _ChallengeScope.ValueType  # 0
    RECOVER: _ChallengeScope.ValueType  # 1

class ChallengeScope(_ChallengeScope, metaclass=_ChallengeScopeEnumTypeWrapper):
    """Operation is the challenge associated with?"""

REGISTER: ChallengeScope.ValueType  # 0
RECOVER: ChallengeScope.ValueType  # 1
global___ChallengeScope = ChallengeScope

@typing_extensions.final
class ChallengeRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SCOPE_FIELD_NUMBER: builtins.int
    NODE_ID_FIELD_NUMBER: builtins.int
    scope: global___ChallengeScope.ValueType
    node_id: builtins.bytes
    def __init__(
        self,
        *,
        scope: global___ChallengeScope.ValueType = ...,
        node_id: builtins.bytes = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["node_id", b"node_id", "scope", b"scope"]) -> None: ...

global___ChallengeRequest = ChallengeRequest

@typing_extensions.final
class ChallengeResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHALLENGE_FIELD_NUMBER: builtins.int
    challenge: builtins.bytes
    def __init__(
        self,
        *,
        challenge: builtins.bytes = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["challenge", b"challenge"]) -> None: ...

global___ChallengeResponse = ChallengeResponse

@typing_extensions.final
class RegistrationRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NODE_ID_FIELD_NUMBER: builtins.int
    BIP32_KEY_FIELD_NUMBER: builtins.int
    NETWORK_FIELD_NUMBER: builtins.int
    CHALLENGE_FIELD_NUMBER: builtins.int
    SIGNATURE_FIELD_NUMBER: builtins.int
    SIGNER_PROTO_FIELD_NUMBER: builtins.int
    INIT_MSG_FIELD_NUMBER: builtins.int
    CSR_FIELD_NUMBER: builtins.int
    INVITE_CODE_FIELD_NUMBER: builtins.int
    STARTUPMSGS_FIELD_NUMBER: builtins.int
    node_id: builtins.bytes
    """33 bytes node public key."""
    bip32_key: builtins.bytes
    """DEPRECATED: The `init_msg` subsumes this field"""
    network: builtins.str
    """Which network is this node going to run on? Options are
    bitcoin, testnet, and regtest.
    """
    challenge: builtins.bytes
    """An previously unused challenge as retrieved from
    `Scheduler.GetChallenge() with `scope=REGISTER`. In
    combination with the `signature` below this is used to
    authenticate the caller and ensure the caller has access to
    the secret keys corresponding to the `node_id`.
    """
    signature: builtins.bytes
    """A signature for the `challenge` signed by the secret key
    corresponding to the `node_id`. Please refer to the
    documentation of `Scheduler.GetChallenge()` for details on
    how to create this signature.
    """
    signer_proto: builtins.str
    """The signer_proto is required in order to determine which
    version the node should run. If these don't match the
    signer may not be able to sign incoming requests.
    """
    init_msg: builtins.bytes
    """The fuil init message returned by the `libhsmd`, this
    supersedes the bip32_key field which was a misnomer. Notice
    that this includes the prefix 0x006F which is the message
    type.
    """
    csr: builtins.bytes
    """The certificate signing request that will be signed by the
    greenlight backend. Notice that this must have the valid
    CN corresponding to the node_id e.g. /users/{node_id} set.
    """
    invite_code: builtins.str
    """An optional invite code. We may want to throttle the
    registration rate. Therefore we might check that a registration
    request has a valid invite code.
    """
    @property
    def startupmsgs(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___StartupMessage]:
        """Messages stashed at the scheduler to allow signerless
        startups.
        """
    def __init__(
        self,
        *,
        node_id: builtins.bytes = ...,
        bip32_key: builtins.bytes = ...,
        network: builtins.str = ...,
        challenge: builtins.bytes = ...,
        signature: builtins.bytes = ...,
        signer_proto: builtins.str = ...,
        init_msg: builtins.bytes = ...,
        csr: builtins.bytes = ...,
        invite_code: builtins.str = ...,
        startupmsgs: collections.abc.Iterable[global___StartupMessage] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["bip32_key", b"bip32_key", "challenge", b"challenge", "csr", b"csr", "init_msg", b"init_msg", "invite_code", b"invite_code", "network", b"network", "node_id", b"node_id", "signature", b"signature", "signer_proto", b"signer_proto", "startupmsgs", b"startupmsgs"]) -> None: ...

global___RegistrationRequest = RegistrationRequest

@typing_extensions.final
class RegistrationResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DEVICE_CERT_FIELD_NUMBER: builtins.int
    DEVICE_KEY_FIELD_NUMBER: builtins.int
    device_cert: builtins.str
    """Upon registering the user receives back the signed certificate that
    belongs to the certificate signing request the that was sent in the
    registration request, so they can authenticate themselves in the future.
    """
    device_key: builtins.str
    """The private key that was used to create the certificate with. This key
    is used to sign the requests to the node.
    """
    def __init__(
        self,
        *,
        device_cert: builtins.str = ...,
        device_key: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["device_cert", b"device_cert", "device_key", b"device_key"]) -> None: ...

global___RegistrationResponse = RegistrationResponse

@typing_extensions.final
class ScheduleRequest(google.protobuf.message.Message):
    """Ask the scheduler to schedule the node to be run on an available nodelet.

    This will always cause the scheduler to kick into action. If you'd
    like to see if a nodelet is currently taking care of this node, or
    wait for one to start please use the
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NODE_ID_FIELD_NUMBER: builtins.int
    node_id: builtins.bytes
    def __init__(
        self,
        *,
        node_id: builtins.bytes = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["node_id", b"node_id"]) -> None: ...

global___ScheduleRequest = ScheduleRequest

@typing_extensions.final
class NodeInfoRequest(google.protobuf.message.Message):
    """Discovery request asking the scheduler if a nodelet is currently assigned
    the specified node_id, or wait for one to be assigned. If `wait` is set to
    `true` the scheduler will keep the request pending until a nodelet is
    assigned.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NODE_ID_FIELD_NUMBER: builtins.int
    WAIT_FIELD_NUMBER: builtins.int
    node_id: builtins.bytes
    wait: builtins.bool
    def __init__(
        self,
        *,
        node_id: builtins.bytes = ...,
        wait: builtins.bool = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["node_id", b"node_id", "wait", b"wait"]) -> None: ...

global___NodeInfoRequest = NodeInfoRequest

@typing_extensions.final
class NodeInfoResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NODE_ID_FIELD_NUMBER: builtins.int
    GRPC_URI_FIELD_NUMBER: builtins.int
    node_id: builtins.bytes
    grpc_uri: builtins.str
    def __init__(
        self,
        *,
        node_id: builtins.bytes = ...,
        grpc_uri: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["grpc_uri", b"grpc_uri", "node_id", b"node_id"]) -> None: ...

global___NodeInfoResponse = NodeInfoResponse

@typing_extensions.final
class RecoveryRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CHALLENGE_FIELD_NUMBER: builtins.int
    SIGNATURE_FIELD_NUMBER: builtins.int
    NODE_ID_FIELD_NUMBER: builtins.int
    CSR_FIELD_NUMBER: builtins.int
    challenge: builtins.bytes
    signature: builtins.bytes
    node_id: builtins.bytes
    csr: builtins.bytes
    """The certificate signing request that will be signed by the
    greenlight backend. Notice that this must have the valid
    CN corresponding to the node_id e.g. /users/{node_id} set.
    """
    def __init__(
        self,
        *,
        challenge: builtins.bytes = ...,
        signature: builtins.bytes = ...,
        node_id: builtins.bytes = ...,
        csr: builtins.bytes = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["challenge", b"challenge", "csr", b"csr", "node_id", b"node_id", "signature", b"signature"]) -> None: ...

global___RecoveryRequest = RecoveryRequest

@typing_extensions.final
class RecoveryResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    DEVICE_CERT_FIELD_NUMBER: builtins.int
    DEVICE_KEY_FIELD_NUMBER: builtins.int
    device_cert: builtins.str
    device_key: builtins.str
    def __init__(
        self,
        *,
        device_cert: builtins.str = ...,
        device_key: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["device_cert", b"device_cert", "device_key", b"device_key"]) -> None: ...

global___RecoveryResponse = RecoveryResponse

@typing_extensions.final
class UpgradeRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SIGNER_VERSION_FIELD_NUMBER: builtins.int
    INITMSG_FIELD_NUMBER: builtins.int
    STARTUPMSGS_FIELD_NUMBER: builtins.int
    signer_version: builtins.str
    """The version of the signer, i.e., the maximum version of the
    protocol that this signer can understand.
    """
    initmsg: builtins.bytes
    """The new initmsg matching the above version. Necessary to
    schedule the node without a signer present.
    Deprecated: Replaced by the more generic `startupmsgs`
    """
    @property
    def startupmsgs(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___StartupMessage]:
        """Messages stashed at the scheduler to allow signerless startups."""
    def __init__(
        self,
        *,
        signer_version: builtins.str = ...,
        initmsg: builtins.bytes = ...,
        startupmsgs: collections.abc.Iterable[global___StartupMessage] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["initmsg", b"initmsg", "signer_version", b"signer_version", "startupmsgs", b"startupmsgs"]) -> None: ...

global___UpgradeRequest = UpgradeRequest

@typing_extensions.final
class UpgradeResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    OLD_VERSION_FIELD_NUMBER: builtins.int
    old_version: builtins.str
    """The version of the node before the upgrade request has been
    processed.
    """
    def __init__(
        self,
        *,
        old_version: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["old_version", b"old_version"]) -> None: ...

global___UpgradeResponse = UpgradeResponse

@typing_extensions.final
class StartupMessage(google.protobuf.message.Message):
    """A message that we know will be requested by `lightningd` at
    startup, and that we stash a response to on the scheduler. This
    allows the scheduler to start a node without requiring the signer
    to connect first. Messages are stored in full, including type
    prefix, but without the length prefix.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    REQUEST_FIELD_NUMBER: builtins.int
    RESPONSE_FIELD_NUMBER: builtins.int
    request: builtins.bytes
    response: builtins.bytes
    def __init__(
        self,
        *,
        request: builtins.bytes = ...,
        response: builtins.bytes = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["request", b"request", "response", b"response"]) -> None: ...

global___StartupMessage = StartupMessage

@typing_extensions.final
class ListInviteCodesRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___ListInviteCodesRequest = ListInviteCodesRequest

@typing_extensions.final
class ListInviteCodesResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    INVITE_CODE_LIST_FIELD_NUMBER: builtins.int
    @property
    def invite_code_list(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___InviteCode]: ...
    def __init__(
        self,
        *,
        invite_code_list: collections.abc.Iterable[global___InviteCode] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["invite_code_list", b"invite_code_list"]) -> None: ...

global___ListInviteCodesResponse = ListInviteCodesResponse

@typing_extensions.final
class InviteCode(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    CODE_FIELD_NUMBER: builtins.int
    IS_REDEEMED_FIELD_NUMBER: builtins.int
    code: builtins.str
    is_redeemed: builtins.bool
    def __init__(
        self,
        *,
        code: builtins.str = ...,
        is_redeemed: builtins.bool = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["code", b"code", "is_redeemed", b"is_redeemed"]) -> None: ...

global___InviteCode = InviteCode

@typing_extensions.final
class ExportNodeRequest(google.protobuf.message.Message):
    """Empty message for now, node identity is extracted from the mTLS
    certificate used to authenticate against the Scheduler.
    """

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___ExportNodeRequest = ExportNodeRequest

@typing_extensions.final
class ExportNodeResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    URL_FIELD_NUMBER: builtins.int
    url: builtins.str
    """URL where the encrypted backup can be retrieved from."""
    def __init__(
        self,
        *,
        url: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal["url", b"url"]) -> None: ...

global___ExportNodeResponse = ExportNodeResponse

@typing_extensions.final
class SignerRejection(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    MSG_FIELD_NUMBER: builtins.int
    REQUEST_FIELD_NUMBER: builtins.int
    GIT_VERSION_FIELD_NUMBER: builtins.int
    msg: builtins.str
    """A human-readable description of what went wrong"""
    @property
    def request(self) -> greenlight_pb2.HsmRequest: ...
    git_version: builtins.str
    def __init__(
        self,
        *,
        msg: builtins.str = ...,
        request: greenlight_pb2.HsmRequest | None = ...,
        git_version: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal["request", b"request"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal["git_version", b"git_version", "msg", b"msg", "request", b"request"]) -> None: ...

global___SignerRejection = SignerRejection
