# Prediction Oracle

The oracle contract returns latest price data using band protocol.

## InitMsg

```json
{
  "band_oracle": "secret...",
  "band_oracle_code_hash": "123",
  "base_symbol": "SCRT",
  "quote_symbol": "USD"
}
```

## HandleMsg

No handle messages

## QueryMsg

### `config`

```json
{
  "config": {}
}
```

### `priceData`

```json
{
  "query_latest_price": {}
}
```
