namespace Aleo.Primitives;

public class Address : IEquatable<Address>
{
    public string Bech32 { get; }

    public Address(string bech32)
    {
        ArgumentNullException.ThrowIfNull(bech32);
        if (!IsValid(bech32))
            throw new ArgumentException($"Invalid Aleo address format: '{bech32}'", nameof(bech32));
        Bech32 = bech32;
    }

    public override string ToString() => Bech32;

    public override bool Equals(object? obj) => Equals(obj as Address);

    public bool Equals(Address? other) =>
        other is not null && string.Equals(Bech32, other.Bech32, StringComparison.Ordinal);

    public override int GetHashCode() => StringComparer.Ordinal.GetHashCode(Bech32);

    public static bool operator ==(Address? left, Address? right) =>
        Equals(left, right);

    public static bool operator !=(Address? left, Address? right) =>
        !Equals(left, right);

    public static bool IsValid(string value)
    {
        if (string.IsNullOrEmpty(value))
            return false;
        if (!value.StartsWith("aleo1", StringComparison.Ordinal))
            return false;
        if (value.Length < 10 || value.Length > 128)
            return false;
        return value.All(c => char.IsAsciiLetterOrDigit(c) || c == '1');
    }

    public static bool TryParse(string value, out Address? result)
    {
        if (!IsValid(value))
        {
            result = null;
            return false;
        }
        result = new Address(value);
        return true;
    }
}
