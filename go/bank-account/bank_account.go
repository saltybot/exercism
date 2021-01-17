package account

import "sync"

type Account struct {
	is_open bool
	balance int64
	lock sync.Mutex
}

func Open(initialDeposit int64) *Account {
	if initialDeposit < 0 {
		return nil
	}
	return &Account{is_open: true, balance: initialDeposit}
}

func (acct *Account) Close() (payout int64, ok bool) {
	acct.lock.Lock()
	defer acct.lock.Unlock()

	if !acct.is_open {
		return 0, false
	}

	acct.is_open = false
	payout = acct.balance
	acct.balance = 0

	return payout, true
}

func (acct *Account) Balance() (balance int64, ok bool) {
	acct.lock.Lock()
	defer acct.lock.Unlock()

	if !acct.is_open {
		return 0, false
	}

	return acct.balance, true
}

func (acct *Account) Deposit(amount int64) (newBalance int64, ok bool) {
	acct.lock.Lock()
	defer acct.lock.Unlock()

	if !acct.is_open {
		return 0, false
	}

	newBalance = acct.balance + amount
	if newBalance < 0 {
		return acct.balance, false
	}

	acct.balance = newBalance
	return acct.balance, true
}