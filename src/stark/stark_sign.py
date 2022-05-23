from request_helpers import iso_to_epoch_seconds
from starkex.order import SignableOrder
from starkex.withdrawal import SignableWithdrawal


def sign_order(network_id, maraket, side, position_id, human_size, human_price, limit_fee, client_id, expiration_epoch_seconds, private_key):
    order = SignableOrder(network_id, maraket, side, position_id, human_size, human_price, limit_fee, client_id, expiration_epoch_seconds)
    signature = order.sign(private_key)
    return signature

def sign_withdraw(network_id, position_id, human_size, client_id, expiration_epoch_seconds, private_key):
    withdraw = SignableWithdrawal(network_id, position_id, human_size, client_id, expiration_epoch_seconds)
    signature = withdraw.sign(private_key)
    return signature