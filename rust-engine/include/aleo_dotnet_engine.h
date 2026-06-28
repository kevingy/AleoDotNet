#ifndef ALEO_DOTNET_ENGINE_H
#define ALEO_DOTNET_ENGINE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Convert AleoError to FFI error code
 */
typedef enum AleoErrorCode {
  Success = 0,
  InvalidInput = 1,
  MemoryAllocationFailed = 2,
  CryptoError = 3,
  SerializationError = 4,
  InvalidKeyFormat = 5,
  InvalidAddressFormat = 6,
  SignatureVerificationFailed = 7,
  ProofGenerationFailed = 8,
  RecordCryptoError = 9,
  Unknown = 99,
} AleoErrorCode;

/**
 * FFI-safe representation of an Aleo address
 *
 * # Note
 * This can either be a bech32 string or fixed bytes depending on the chosen representation.
 * The current implementation uses a bech32 string for flexibility.
 */
typedef struct AleoAddress {
  /**
   * Null-terminated bech32 address string
   * This pointer is owned by Rust and must be freed using aleo_free_address
   */
  char *bech32;
} AleoAddress;

/**
 * FFI-safe representation of an Aleo record
 *
 * # Note
 * Records are complex structures. This is a simplified placeholder.
 */
typedef struct AleoRecord {
  /**
   * Serialized record data
   * This pointer is owned by Rust and must be freed
   */
  uint8_t *data;
  /**
   * Length of the data
   */
  uintptr_t len;
} AleoRecord;

/**
 * FFI-safe representation of a transaction
 *
 * # Note
 * Transactions are complex structures. This is a simplified placeholder.
 */
typedef struct AleoTransaction {
  /**
   * Serialized transaction data
   * This pointer is owned by Rust and must be freed
   */
  uint8_t *data;
  /**
   * Length of the data
   */
  uintptr_t len;
} AleoTransaction;

/**
 * FFI-safe representation of an Aleo private key
 *
 * # Note
 * The exact size and layout must match Aleo's private key specification.
 * snarkVM private keys are 32 bytes.
 */
typedef struct AleoPrivateKey {
  /**
   * Raw bytes of the private key (32 bytes for snarkVM)
   */
  uint8_t bytes[32];
} AleoPrivateKey;

/**
 * FFI-safe representation of an Aleo view key
 *
 * # Note
 * The exact size and layout must match Aleo's view key specification.
 */
typedef struct AleoViewKey {
  /**
   * Raw bytes of the view key
   * TODO: Determine correct size based on Aleo protocol
   */
  uint8_t bytes[32];
} AleoViewKey;

/**
 * Free a string allocated by Rust
 */
void aleo_free_string(char *ptr);

const char *aleo_get_last_error(void);

/**
 * Allocate memory of a given size
 *
 * # Safety
 * Caller must ensure the allocated memory is properly freed using aleo_free
 */
void *aleo_alloc(uintptr_t size);

/**
 * Free memory allocated by aleo_alloc
 *
 * # Safety
 * ptr must have been allocated by aleo_alloc
 */
void aleo_free(void *ptr);

/**
 * Free bytes allocated by alloc_bytes
 */
void aleo_free_bytes(uint8_t *ptr, uintptr_t size);

/**
 * Free an AleoAddress
 *
 * # Safety
 * Caller must ensure `addr` was allocated by this library and is not used after this call.
 */
void aleo_free_address(struct AleoAddress *addr);

/**
 * Free an AleoRecord
 */
void aleo_free_record(struct AleoRecord *record);

/**
 * Free an AleoTransaction
 */
void aleo_free_transaction(struct AleoTransaction *tx);

/**
 * Free an AleoPrivateKey
 */
void aleo_free_private_key(struct AleoPrivateKey *key);

/**
 * Free an AleoViewKey
 */
void aleo_free_view_key(struct AleoViewKey *key);

/**
 * Generate a new private key
 *
 * # Safety
 * Caller must free the private key using aleo_free_private_key
 */
enum AleoErrorCode aleo_generate_private_key(struct AleoPrivateKey **out_key);

/**
 * Derive view key from private key
 *
 * # Safety
 * Caller must free the view key using aleo_free_view_key
 */
enum AleoErrorCode aleo_derive_view_key(const struct AleoPrivateKey *private_key_ptr,
                                        struct AleoViewKey **out_view_key);

/**
 * Derive address from view key
 *
 * # Safety
 * Caller must free the address using aleo_free_address
 */
enum AleoErrorCode aleo_derive_address(const struct AleoViewKey *view_key_ptr,
                                       struct AleoAddress **out_address);

/**
 * Parse address from string
 *
 * # Arguments
 * * `str` - Null-terminated string containing the address
 * * `out_address` - Pointer to receive the allocated address
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the address using aleo_free_address
 */
enum AleoErrorCode aleo_address_from_string(const char *str, struct AleoAddress **out_address);

/**
 * Convert address to string
 *
 * # Arguments
 * * `addr` - The address to convert
 * * `out_string` - Pointer to receive the allocated string
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the string using aleo_free_string
 */
enum AleoErrorCode aleo_address_to_string(const struct AleoAddress *addr, char **out_string);

/**
 * Build a transfer transaction
 *
 * # Arguments
 * * `private_key` - The private key for signing
 * * `recipient` - The recipient address
 * * `amount` - The amount to transfer (in microcredits)
 * * `out_transaction` - Pointer to receive the allocated transaction
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the transaction using aleo_free_transaction
 */
enum AleoErrorCode aleo_build_transfer(const struct AleoPrivateKey *private_key,
                                       const struct AleoAddress *recipient,
                                       uint64_t amount,
                                       struct AleoTransaction **out_transaction);

/**
 * Sign a transaction
 *
 * # Arguments
 * * `private_key` - The private key for signing
 * * `transaction` - The transaction data to sign (as a string)
 * * `out_signed_transaction` - Pointer to receive the allocated signed transaction string
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the string using aleo_free_string
 */
enum AleoErrorCode aleo_sign_transaction(const struct AleoPrivateKey *private_key,
                                         const char *transaction,
                                         char **out_signed_transaction);

/**
 * Decrypt a record
 *
 * # Arguments
 * * `view_key` - The view key for decryption
 * * `encrypted_record` - The encrypted record data (as a string)
 * * `out_decrypted_record` - Pointer to receive the allocated `AleoRecord`
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the returned `AleoRecord` using `aleo_free_record`
 */
enum AleoErrorCode aleo_decrypt_record(const struct AleoViewKey *view_key,
                                       const char *encrypted_record,
                                       struct AleoRecord **out_decrypted_record);

/**
 * Encrypt a record
 *
 * # Arguments
 * * `recipient` - The recipient address
 * * `plaintext_record` - The plaintext record data (as a string)
 * * `out_encrypted_record` - Pointer to receive the allocated encrypted record string
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the string using aleo_free_string
 */
enum AleoErrorCode aleo_encrypt_record(const struct AleoAddress *recipient,
                                       const char *plaintext_record,
                                       char **out_encrypted_record);

/**
 * Convert a record to a JSON string
 *
 * The record data bytes are expected to be valid UTF-8 JSON content
 * (e.g. from a prior call to `aleo_decrypt_record`).
 *
 * # Arguments
 * * `record` - Pointer to an `AleoRecord`
 * * `out_string` - Pointer to receive the allocated JSON string
 *
 * # Returns
 * Error code (0 on success)
 *
 * # Safety
 * Caller must free the returned string using `aleo_free_string`
 */
enum AleoErrorCode aleo_record_to_json(const struct AleoRecord *record, char **out_string);

#endif /* ALEO_DOTNET_ENGINE_H */
