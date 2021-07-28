# Price Prediction

Price prediction contract for users to bet price changes in each round.

## InitMsg

```json
{
  "operator_addr": "secret...",
  "treasury_addr": "secret...",
  "bet_asset": {
    "native_token": {
      "denom": "uscrt"
    }
  },
  "oracle_addr": "secret...",
  "oracle_code_hash": "123...",
  "fee_rate": "0.3",
  "interval": "600",
  "grace_interval": "300"
}
```

## HandleMsg

### `update_config`

The owner can update configuration

```json
{
  "update_config":
  {
    "owner_addr": Option<HumanAddr>,
    "operator_addr": Option<HumanAddr>,"treasury_addr": Option<HumanAddr>,"oracle_addr": Option<HumanAddr>,"oracle_code_hash": Option<String>,
    "fee_rate": Option<Decimal>,
    "interval": Option<u64>,
    "grace_interval": Option<u64>,
  }
}
```

### `bet`

The user can bet to `UP` or `DOWN` for next round.

```json
{
  "bet": {
    "position": "UP"
  }
}
```

### `claim`

Winners can claim reward of ended rounds.

```json
{
  "claim": {
    "epoch": "1"
  }
}
```

### `execute_round`

Operator executes current round for prediction results.

```json
{
  "execute_round": {}
}
```

### `withdraw`

Withdraw performance fee to treasury address

```json
{
  "withdraw": {}
}
```

### `pause`

Owner can pause prediction game in unexpected cases

```json
{
  "pause": {}
}
```

### `start_genesis_round`

Owner starts genesis round which is not bettable. From next rounds, users can able to bet. Only executable when prediction has been paused.

```json
{
  "start_genesis_round": {}
}
```

## QueryMsg

### `config`

```json
{
  "config": {}
}
```

### `state`

```json
{
  "state": {}
}
```

### `round`

```json
{
  "round": {
    "epoch": "1"
  }
}
```

### `bet`

```json
{
  "bet": {
    "epoch": "1",
    "user": "secret..."
  }
}
```
