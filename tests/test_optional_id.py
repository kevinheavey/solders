from solders.rpc.requests import GetClusterNodes

def test_optional_id() -> None:
    req1 = GetClusterNodes()
    req2 = GetClusterNodes(id=42)
    assert req1.id == 0
    assert req2.id == 42
