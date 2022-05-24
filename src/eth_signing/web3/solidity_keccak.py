from typing import (Any, List)
from eth_typing import HexStr
from eth_typing.abi import TypeStr
from abi import (
    is_address_type,
    is_array_type,
    is_bool_type,
    is_bytes_type,
    is_int_type,
    is_string_type,
    is_uint_type,
    size_of_type,
    sub_type_of_array_type,
)

text_types = (str,)

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

def is_text(value: Any) -> bool:
    return isinstance(value, text_types)

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
    
def abi_ens_resolver(w3: "Web3", type_str: TypeStr, val: Any) -> Tuple[TypeStr, Any]:
    if type_str == 'address' and is_ens_name(val):
        if w3 is None:
            raise InvalidAddress(
                f"Could not look up name {val!r} because no web3"
                " connection available"
            )
        elif w3.ens is None:
            raise InvalidAddress(
                f"Could not look up name {val!r} because ENS is"
                " set to None"
            )
        elif int(w3.net.version) != 1 and not isinstance(w3.ens, StaticENS):
            raise InvalidAddress(
                f"Could not look up name {val!r} because web3 is"
                " not connected to mainnet"
            )
        else:
            return type_str, validate_name_has_address(w3.ens, val)
    else:
        return type_str, val