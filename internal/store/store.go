package store

import (
	"context"
	"errors"
)

var ErrNotFound = errors.New("key not found")

type KeyValue struct {
	Key string
	Value []byte
}

type Store interface {
	Get(ctx context.Context, key string) ([]byte, error)
	Put(ctx context.Context, key string, value []byte) error
	Delete(ctx context.Context, key string) error
	List(ctx context.Context, prefix string) ([]KeyValue, error)
}

