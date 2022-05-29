import os

from eth_keys import (
    keys,
)
from eth_keys.exceptions import (
    ValidationError,
)
from eth_utils.curried import (
    combomethod,
)
from hexbytes import (
    HexBytes,
)
from eth_account._utils.signing import (
    sign_message_hash,
)

from eth_account.datastructures import (
    SignedMessage,
)

from eth_account.signers.local import (
    LocalAccount,
)


class Account(object):
    """
    The primary entry point for working with Ethereum private keys.

    It does **not** require a connection to an Ethereum node.
    """
    _keys = keys

    _default_kdf = os.getenv('ETH_ACCOUNT_KDF', 'scrypt')

    # Enable unaudited features (off by default)
    _use_unaudited_hdwallet_features = False

    @combomethod
    def from_key(self, private_key):
        r"""
        Returns a convenient object for working with the given private key.

        :param private_key: The raw private key
        :type private_key: hex str, bytes, int or :class:`eth_keys.datatypes.PrivateKey`
        :return: object with methods for signing and encrypting
        :rtype: LocalAccount

        .. doctest:: python

            >>> acct = Account.from_key(
            ... 0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364)
            >>> acct.address
            '0x5ce9454909639D2D17A3F753ce7d93fa0b9aB12E'
            >>> acct.key
            HexBytes('0xb25c7db31feed9122727bf0939dc769a96564b2de4c4726d035b36ecf1e5b364')

            # These methods are also available: sign_message(), sign_transaction(), encrypt()
            # They correspond to the same-named methods in Account.*
            # but without the private key argument
        """
        key = self._parsePrivateKey(private_key)
        return LocalAccount(key, self)


    @combomethod
    def _sign_hash(self, message_hash, private_key):
        msg_hash_bytes = HexBytes(message_hash)
        if len(msg_hash_bytes) != 32:
            raise ValueError("The message hash must be exactly 32-bytes")

        key = self._parsePrivateKey(private_key)

        (v, r, s, eth_signature_bytes) = sign_message_hash(key, msg_hash_bytes)
        return SignedMessage(
            messageHash=msg_hash_bytes,
            r=r,
            s=s,
            v=v,
            signature=HexBytes(eth_signature_bytes),
        )


    @combomethod
    def _parsePrivateKey(self, key):
        """
        Generate a :class:`eth_keys.datatypes.PrivateKey` from the provided key.

        If the key is already of type :class:`eth_keys.datatypes.PrivateKey`, return the key.

        :param key: the private key from which a :class:`eth_keys.datatypes.PrivateKey`
                    will be generated
        :type key: hex str, bytes, int or :class:`eth_keys.datatypes.PrivateKey`
        :returns: the provided key represented as a :class:`eth_keys.datatypes.PrivateKey`
        """
        if isinstance(key, self._keys.PrivateKey):
            return key

        try:
            return self._keys.PrivateKey(HexBytes(key))
        except ValidationError as original_exception:
            raise ValueError(
                "The private key must be exactly 32 bytes long, instead of "
                "%d bytes." % len(key)
            ) from original_exception
