===========
RPC helpers
===========

While ``solders`` doesn't talk to RPCs directly, it does help to build RPC request JSONs
and parse RPC responses. It even supports batch requests (calling multiple RPC methods in one go).

-----------------------
Building an RPC request
-----------------------

Here we build a ``getFeeForMessage`` request and convert it to a JSON string
using the ``.to_json()`` method:

.. doctest::

    >>> from solders.rpc.requests import GetFeeForMessage
    >>> from solders.commitment_config import CommitmentLevel
    >>> from solders.message import MessageV0
    >>> GetFeeForMessage(MessageV0.default(), commitment=CommitmentLevel.Processed).to_json()
    '{"method":"getFeeForMessage","jsonrpc":"2.0","id":0,"params":["gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",{"commitment":"processed"}]}'

----------------------------
Building a batch RPC request
----------------------------

We can combine a list of RPC request objects into a single JSON request using
the ``batch_to_json`` function:

.. doctest::

    >>> from solders.rpc.requests import batch_to_json, GetClusterNodes, GetEpochSchedule
    >>> batch_to_json([GetClusterNodes(0), GetEpochSchedule(1)])
    '[{"method":"getClusterNodes","jsonrpc":"2.0","id":0},{"method":"getEpochSchedule","jsonrpc":"2.0","id":1}]'

-----------------------
Parsing an RPC response
-----------------------

The ``rpc.repsonses`` module parses JSON RPC responses into strongly-typed objects:

.. testcode::

    from solders.rpc.responses import GetStakeActivationResp, RpcStakeActivation, StakeActivationState
    raw = """{
    "jsonrpc": "2.0",
    "result": { "active": 197717120, "inactive": 0, "state": "active" },
    "id": 1
    }"""
    parsed = GetStakeActivationResp.from_json(raw)
    assert isinstance(parsed, GetStakeActivationResp)
    assert parsed.value == RpcStakeActivation(
        state=StakeActivationState.Active, active=197717120, inactive=0
    )

-----------------------------
Parsing an RPC batch repsonse
-----------------------------

The ``batch_from_json`` function parses an RPC batch response into a list of objects:

.. doctest::

    >>> from solders.rpc.responses import batch_from_json, GetBlockHeightResp, GetFirstAvailableBlockResp
    >>> raw = '[{ "jsonrpc": "2.0", "result": 1233, "id": 1 },{ "jsonrpc": "2.0", "result": 111, "id": 1 }]'
    >>> batch_from_json(raw, [GetBlockHeightResp, GetFirstAvailableBlockResp])
    [GetBlockHeightResp(
        1233,
    ), GetFirstAvailableBlockResp(
        111,
    )]
