from starkex.order import SignableOrder
from starkex.withdrawal import SignableWithdrawal
from starkex.conditional_transfer import SignableConditionalTransfer
from starkex.transfer import SignableTransfer
from web3 import Web3
from starkex.helpers import nonce_from_client_id

def sign_order(network_id, maraket, side, position_id, human_size, human_price, limit_fee, client_id, expiration_epoch_seconds, private_key):
    order = SignableOrder(network_id, maraket, side, position_id, human_size, human_price, limit_fee, client_id, expiration_epoch_seconds)
    signature = order.sign(private_key)
    return signature

def sign_withdraw(network_id, position_id, human_size, client_id, expiration_epoch_seconds, private_key):
    withdraw = SignableWithdrawal(network_id, position_id, human_size, client_id, expiration_epoch_seconds)
    signature = withdraw.sign(private_key)
    return signature

def sign_fast_withdraw(network_id, sender_position_id, receiver_position_id, receiver_public_key, fact_registry_address, recipient, token_decimals, human_amount, token_address, client_id, expiration_epoch_seconds, private_key):
    fact = get_transfer_erc20_fact(recipient, token_decimals, human_amount, token_address, nonce_from_client_id(client_id),)
    withdraw = SignableConditionalTransfer(network_id, sender_position_id, receiver_position_id, receiver_public_key, fact_registry_address, fact, human_amount, client_id, expiration_epoch_seconds)
    signature = withdraw.sign(private_key)
    return signature

def sign_transfer(network_id, sender_position_id, receiver_position_id, receiver_public_key,  human_amount, client_id, expiration_epoch_seconds, private_key):
    transfer = SignableTransfer(network_id, sender_position_id, receiver_position_id, receiver_public_key, human_amount, client_id, expiration_epoch_seconds)
    signature = transfer.sign(private_key)
    return signature


def get_transfer_erc20_fact(
    recipient,
    token_decimals,
    human_amount,
    token_address,
    salt,
):
    token_amount = float(human_amount) * (10 ** token_decimals)
    if not token_amount.is_integer():
        raise ValueError(
            'Amount {} has more precision than token decimals {}'.format(
                human_amount,
                token_decimals,
            )
        )
    hex_bytes = Web3.solidityKeccak(
        [
            'address',
            'uint256',
            'address',
            'uint256',
        ],
        [
            recipient,
            int(token_amount),
            token_address,
            salt,
        ],
    )

    return bytes(hex_bytes)