from request_helpers import iso_to_epoch_seconds
from starkex.order import SignableOrder


def sign_order(network_id, maraket, side, position_id, human_size, human_price, limit_fee, client_id, expiration_epoch_seconds, private_key):
    order = SignableOrder(network_id, maraket, side, position_id, human_size, human_price, limit_fee, client_id, expiration_epoch_seconds)
    signature = order.sign(private_key)
    return signature