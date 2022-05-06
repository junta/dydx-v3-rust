from request_helpers import iso_to_epoch_seconds
from starkex.order import SignableOrder

MOCK_PRIVATE_KEY = (
    '58c7d5a90b1776bde86ebac077e053ed85b0f7164f53b080304a531947f46e3'
)

def sign_order(network_id, maraket, order_side):

    order = SignableOrder(network_id, maraket, order_side, 12345,'145.0005','350.00067','0.125','This is an ID that the client came up with to describe this order',  iso_to_epoch_seconds('2020-09-17T04:15:55.028Z'))
    
    signature = order.sign(MOCK_PRIVATE_KEY)
    # print(signature)
    return signature