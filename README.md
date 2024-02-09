Ledgership aka membermower

## Environment
```
LEDGERSHIP_SERVICE_<name>_KEY=<secret>,

LEDGERSHIP_FAILURE_WEBHOOK=https://must_be_set

LEDGERSHIP_ADMIN_GROUPS=k-space:admin # can edit packages
LEDGERSHIP_AUDIT_GROUPS=k-space:accounting # can view everything
LEDGERSHIP_DEALS_GROUPS=k-space:onboarder # can start subscriptionships on behalf of users, with custom price

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
      {"subscription": 159, "data": "k-space:vm_user"}
    ],
    "alice": [
      {"subscription": 160, "data": "k-space:package_member"},
      {"subscription": 161, "data": "k-space:vm_user"},
      {"subscription": 162, "data": "k-space:vm_user"},
      {"subscription": 164, "data": "k-space:workshop"}
    ]
  }
}
---
# Authorization: slack-controller-token GET /grants
{
  "service": "slack",
  "grants": {
    "bob": [
      {"subscription": 158, "data": "friend"}
    ],
    "alice": [
      {"subscription": 160, "data": "member"}
    ]
  }
}
---
# Authorization: proxmox-controller-token GET /grants
{
  "service": "proxmox",
  "grants": {
    "bob": [
      {"subscription": 159, "data": "cpu=4,ram=4096,disk=100"}
    ],
    "alice": [
      {"subscription": 161, "data": "cpu=4,ram=4096,disk=100"},
      {"subscription": 162, "data": "cpu=4,ram=4096,disk=100"}
    ]
  }
}
```
