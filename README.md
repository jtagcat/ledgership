Ledgership aka membermower

## Environment
```
LEDGERSHIP_SERVICE_<name>_KEY=<secret>,
LEDGERSHIP_FAILURE_WEBHOOK=https://must_be_set
LEDGERSHIP_ADMIN_GROUPS=k-space:onboarder,k-space:admin
LEDGERSHIP_AUDIT_GROUPS=k-space:accounting
LEDGERSHIP_MINIMUM_BALANCE=-5.00 EUR # Vending machine convenience
LEDGERSHIP_GRACE_PERIOD=1d # only after first payment
```

## Dependencies
https://docs.rs/openssl/latest/openssl/#automatic

## Interfacing with valid subscriptions
```
# Authorization: oidc-token GET /grants
{
  "service": "oidc",
  "grants": {
    "bob": [
      "k-space:package_supporter",
      "k-space:vm_user"
    ],
    "alice": [
      "k-space:package_member",
      "k-space:vm_user",
      "k-space:vm_user",
      "k-space:workshop"
    ]
  }
}
---
# Authorization: slack-controller-token GET /grants
{
  "service": "slack",
  "grants": {
    "bob": [
      "guest"
    ],
    "alice": [
      "member"
    ]
  }
}
---
# Authorization: proxmox-controller-token GET /grants
{
  "service": "proxmox",
  "grants": {
    "bob": [
      "cpu=4,ram=4096,disk=100"
    ],
    "alice": [
      "cpu=4,ram=4096,disk=100",
      "cpu=4,ram=4096,disk=100"
    ]
  }
}
```
