Ledgership aka membermower

## Roles
```
LEDGERSHIP_SERVICEKEY_<name>=<secret>
LEDGERSHIP_FAILURE_WEBHOOK=https://must_be_set
LEDGERSHIP_ADMIN_GROUPS=k-space:onboarder,k-space:admin
LEDGERSHIP_AUDIT_GROUPS=k-space:accounting
LEDGERSHIP_MINIMUM_BALANCE=-5.00 EUR # Vending machine convenience
LEDGERSHIP_GRACE_PERIOD=1d # only after first payment
```

## Bill
Follow dayofmonth, if DoM is out of range, use last day of month.

## Overdrafting
GET /negative {admin}
GET /negative/:user?period=5d {admin}

## Dependencies
https://docs.rs/openssl/latest/openssl/#automatic
