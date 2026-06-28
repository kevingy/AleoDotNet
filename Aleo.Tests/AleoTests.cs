namespace Aleo.Tests;

public class AleoTests
{
    [Fact]
    public void ScaffoldingTypesCanBeConstructed()
    {
        _ = new Aleo.Primitives.Address("aleo1placeholder");
        _ = new Aleo.Crypto.AleoCrypto();
        _ = new Aleo.Rpc.AleoRpcClient();
        _ = new Aleo.Wallet.AleoWallet();
        _ = new Aleo.Sdk.AleoSdk();
    }
}
