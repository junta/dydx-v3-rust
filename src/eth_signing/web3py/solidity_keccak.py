from typing import (Any, List, Optional, Union, Sequence, Callable, Tuple, TypeVar, Iterable)
from .eth_typing import (
    AnyAddress,
    ChecksumAddress,
    HexStr,
    Primitives,
)
from collections.abc import (
    Mapping,
)
from .eth_typing.abi import TypeStr
from ._utils.abi import (
    is_address_type,
    is_array_type,
    is_bool_type,
    is_bytes_type,
    is_int_type,
    is_string_type,
    is_uint_type,
    size_of_type,
    sub_type_of_array_type,
    is_recognized_type,
    length_of_array_type,
    abi_data_tree,
    data_tree_map,
    strip_abi_type
)
from eth_hash.auto import keccak as keccak_256

from cytoolz import (partial, pipe, curry)
from eth_utils import (
    add_0x_prefix,
    encode_hex,
    is_bytes,
    is_hex,
    is_list_like,
    remove_0x_prefix,
    to_bytes,
    to_hex,
    is_boolean,
    is_string,
    is_integer,
    is_checksum_address,
    is_hex_address,
    is_binary_address,
    is_text,
    decode_hex
)

import itertools
TReturn = TypeVar("TReturn")
bytes_types = (bytes, bytearray)
integer_types = (int,)
text_types = (str,)
string_types = (bytes, str, bytearray)

def solidityKeccak(abi_types: List[TypeStr], values: List[Any]) -> bytes:
    """
    Executes keccak256 exactly as Solidity does.
    Takes list of abi_types as inputs -- `[uint24, int8[], bool]`
    and list of corresponding values  -- `[20, [-1, 5, 0], True]`
    """
    if len(abi_types) != len(values):
        raise ValueError(
            "Length mismatch between provided abi types and values.  Got "
            f"{len(abi_types)} types and {len(values)} values."
        )

    w3 = None

    normalized_values = map_abi_data([abi_ens_resolver(w3)], abi_types, values)

    hex_string = add_0x_prefix(HexStr(''.join(
        remove_0x_prefix(hex_encode_abi_type(abi_type, value))
        for abi_type, value
        in zip(abi_types, normalized_values)
    )))
    return keccak(hexstr=hex_string)

def keccak(primitive: Optional[Primitives] = None, text: Optional[str] = None,
            hexstr: Optional[HexStr] = None) -> bytes:
    if isinstance(primitive, (bytes, int, type(None))):
        input_bytes = to_bytes(primitive, hexstr=hexstr, text=text)
        return eth_utils_keccak(input_bytes)

    raise TypeError(
        f"You called keccak with first arg {primitive!r} and keywords {{'text': {text!r}, "
        f"'hexstr': {hexstr!r}}}. You must call it with one of these approaches: "
        "keccak(text='txt'), keccak(hexstr='0x747874'), keccak(b'\\x74\\x78\\x74'), "
        "or keccak(0x747874)."
    )

def eth_utils_keccak(
    primitive: Union[bytes, int, bool] = None, hexstr: str = None, text: str = None
) -> bytes:
    return keccak_256(to_bytes(primitive, hexstr, text))

def hex_encode_abi_type(abi_type: TypeStr, value: Any,
                        force_size: Optional[int] = None) -> HexStr:
    """
    Encodes value into a hex string in format of abi_type
    """
    validate_abi_type(abi_type)
    validate_abi_value(abi_type, value)

    data_size = force_size or size_of_type(abi_type)
    if is_array_type(abi_type):
        sub_type = sub_type_of_array_type(abi_type)
        return HexStr(
            "".join([remove_0x_prefix(hex_encode_abi_type(sub_type, v, 256)) for v in value])
        )
    elif is_bool_type(abi_type):
        return to_hex_with_size(value, data_size)
    elif is_uint_type(abi_type):
        return to_hex_with_size(value, data_size)
    elif is_int_type(abi_type):
        return to_hex_twos_compliment(value, data_size)
    elif is_address_type(abi_type):
        return pad_hex(value, data_size)
    elif is_bytes_type(abi_type):
        if is_bytes(value):
            return encode_hex(value)
        else:
            return value
    elif is_string_type(abi_type):
        return to_hex(text=value)
    else:
        raise ValueError(
            f"Unsupported ABI type: {abi_type}"
        )


def to_hex_twos_compliment(value: Any, bit_size: int) -> HexStr:
    """
    Converts integer value to twos compliment hex representation with given bit_size
    """
    if value >= 0:
        return to_hex_with_size(value, bit_size)

    value = (1 << bit_size) + value
    hex_value = hex(value)
    hex_value = HexStr(hex_value.rstrip("L"))
    return hex_value

def to_hex_with_size(value: Any, bit_size: int) -> HexStr:
    """
    Converts a value to hex with given bit_size:
    """
    return pad_hex(to_hex(value), bit_size)

def to_hex(
    primitive: Primitives = None, hexstr: HexStr = None, text: str = None
) -> HexStr:
    """
    Auto converts any supported value into its hex representation.
    Trims leading zeros, as defined in:
    https://github.com/ethereum/wiki/wiki/JSON-RPC#hex-value-encoding
    """
    if hexstr is not None:
        return add_0x_prefix(HexStr(hexstr.lower()))

    if text is not None:
        return encode_hex(text.encode("utf-8"))

    if is_boolean(primitive):
        return HexStr("0x1") if primitive else HexStr("0x0")

    if isinstance(primitive, (bytes, bytearray)):
        return encode_hex(primitive)
    elif is_string(primitive):
        raise TypeError(
            "Unsupported type: The primitive argument must be one of: bytes,"
            "bytearray, int or bool and not str"
        )

    if is_integer(primitive):
        return HexStr(hex(primitive))

    raise TypeError(
        "Unsupported type: '{0}'.  Must be one of: bool, str, bytes, bytearray"
        "or int.".format(repr(type(primitive)))
    )

def pad_hex(value: Any, bit_size: int) -> HexStr:
    """
    Pads a hex string up to the given bit_size
    """
    value = remove_0x_prefix(value)
    return add_0x_prefix(value.zfill(int(bit_size / 4)))

def validate_abi_type(abi_type: TypeStr) -> None:
    """
    Helper function for validating an abi_type
    """
    if not is_recognized_type(abi_type):
        raise ValueError(f"Unrecognized abi_type: {abi_type}")

def validate_abi_type(abi_type: TypeStr) -> None:
    """
    Helper function for validating an abi_type
    """
    if not is_recognized_type(abi_type):
        raise ValueError(f"Unrecognized abi_type: {abi_type}")


def validate_abi_value(abi_type: TypeStr, value: Any) -> None:
    """
    Helper function for validating a value against the expected abi_type
    Note: abi_type 'bytes' must either be python3 'bytes' object or ''
    """
    if is_array_type(abi_type) and is_list_like(value):
        # validate length
        specified_length = length_of_array_type(abi_type)
        if specified_length is not None:
            if specified_length < 1:
                raise TypeError(
                    f"Invalid abi-type: {abi_type}. Length of fixed sized "
                    "arrays must be greater than 0."
                )
            if specified_length != len(value):
                raise TypeError(
                    "The following array length does not the length specified"
                    f"by the abi-type, {abi_type}: {value}"
                )

        # validate sub_types
        sub_type = sub_type_of_array_type(abi_type)
        for v in value:
            validate_abi_value(sub_type, v)
        return
    elif is_bool_type(abi_type) and is_boolean(value):
        return
    elif is_uint_type(abi_type) and is_integer(value) and value >= 0:
        return
    elif is_int_type(abi_type) and is_integer(value):
        return
    elif is_address_type(abi_type):
        validate_address(value)
        return
    elif is_bytes_type(abi_type):
        if is_bytes(value):
            return
        elif is_string(value):
            if is_0x_prefixed(value):
                return
            else:
                raise TypeError(
                    "ABI values of abi-type 'bytes' must be either"
                    "a python3 'bytes' object or an '0x' prefixed string."
                )
    elif is_string_type(abi_type) and is_string(value):
        return

    raise TypeError(
        f"The following abi value is not a '{abi_type}': {value}"
    )


def is_not_address_string(value: Any) -> bool:
    return (is_string(value) and not is_bytes(value) and not
            is_checksum_address(value) and not is_hex_address(value))

def validate_address(value: Any) -> None:
    """
    Helper function for validating an address
    """
    if is_bytes(value):
        if not is_binary_address(value):
            raise InvalidAddress("Address must be 20 bytes when input type is bytes", value)
        return

    if not isinstance(value, str):
        raise TypeError(f'Address {value} must be provided as a string')
    if not is_hex_address(value):
        raise InvalidAddress("Address must be 20 bytes, as a hex string with a 0x prefix", value)
    if not is_checksum_address(value):
        if value == value.lower():
            raise InvalidAddress(
                "Web3.py only accepts checksum addresses. "
                "The software that gave you this non-checksum address should be considered unsafe, "
                "please file it as a bug on their platform. "
                "Try using an ENS name instead. Or, if you must accept lower safety, "
                "use Web3.toChecksumAddress(lower_case_address).",
                value,
            )
        else:
            raise InvalidAddress(
                "Address has an invalid EIP-55 checksum. "
                "After looking up the address from the original source, try again.",
                value,
            )

def remove_0x_prefix(value: HexStr) -> HexStr:
    if is_0x_prefixed(value):
        return HexStr(value[2:])
    return value

def add_0x_prefix(value: HexStr) -> HexStr:
    if is_0x_prefixed(value):
        return value
    return HexStr("0x" + value)

def is_0x_prefixed(value: str) -> bool:
    if not is_text(value):
        raise TypeError(
            "is_0x_prefixed requires text typed arguments. Got: {0}".format(repr(value))
        )
    return value.startswith(("0x", "0X"))

def to_bytes(
    primitive: Primitives = None, hexstr: HexStr = None, text: str = None
) -> bytes:
    if is_boolean(primitive):
        return b"\x01" if primitive else b"\x00"
    elif isinstance(primitive, bytearray):
        return bytes(primitive)
    elif isinstance(primitive, bytes):
        return primitive
    elif is_integer(primitive):
        return to_bytes(hexstr=to_hex(primitive))
    elif hexstr is not None:
        if len(hexstr) % 2:
            # type check ignored here because casting an Optional arg to str is not possible
            hexstr = "0x0" + remove_0x_prefix(hexstr)  # type: ignore
        return decode_hex(hexstr)
    elif text is not None:
        return text.encode("utf-8")
    raise TypeError(
        "expected a bool, int, byte or bytearray in first arg, or keyword of hexstr or text"
    )

def map_abi_data(
    normalizers: Sequence[Callable[[TypeStr, Any], Tuple[TypeStr, Any]]],
    types: Sequence[TypeStr],
    data: Sequence[Any],
) -> Any:
    """
    This function will apply normalizers to your data, in the
    context of the relevant types. Each normalizer is in the format:

    def normalizer(datatype, data):
        # Conditionally modify data
        return (datatype, data)

    Where datatype is a valid ABI type string, like "uint".

    In case of an array, like "bool[2]", normalizer will receive `data`
    as an iterable of typed data, like `[("bool", True), ("bool", False)]`.

    Internals
    ---

    This is accomplished by:

    1. Decorating the data tree with types
    2. Recursively mapping each of the normalizers to the data
    3. Stripping the types back out of the tree
    """
    pipeline = itertools.chain(
        [abi_data_tree(types)],
        map(data_tree_map, normalizers),
        [partial(recursive_map, strip_abi_type)],
    )

    return pipe(data, *pipeline)

def recursive_map(func: Callable[..., TReturn], data: Any) -> TReturn:
    """
    Apply func to data, and any collection items inside data (using map_collection).
    Define func so that it only applies to the type of value that you want it to apply to.
    """
    def recurse(item: Any) -> TReturn:
        return recursive_map(func, item)
    items_mapped = map_collection(recurse, data)
    return func(items_mapped)

def map_collection(func: Callable[..., TReturn], collection: Any) -> Any:
    """
    Apply func to each element of a collection, or value of a dictionary.
    If the value is not a collection, return it unmodified
    """
    datatype = type(collection)
    if isinstance(collection, Mapping):
        return datatype((key, func(val)) for key, val in collection.items())
    if is_string(collection):
        return collection
    elif isinstance(collection, Iterable):
        return datatype(map(func, collection))
    else:
        return collection

@curry
def abi_ens_resolver(w3: "Web3", type_str: TypeStr, val: Any) -> Tuple[TypeStr, Any]:
    return type_str, val

class InvalidAddress(ValueError):
    """
    The supplied address does not have a valid checksum, as defined in EIP-55
    """
    pass