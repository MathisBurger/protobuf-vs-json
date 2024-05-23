# Serialization

## Small

### 1000 Entries

JSON: 2.8531ms
Protobuf: 1.6126ms (76,9% faster)

### 1.000.000 Entries

JSON: 2.9304s
Protobuf: 1.6648s (76% faster)

## Mid

### 1000 Entries

JSON: 9.6198ms
Protobuf: 2.9380ms (227% faster)

### 1.000.000 Entries

JSON: 9.6444s
Protobuf: 3.0122s (220% faster)

## Large

### 1000 Entries

JSON: 2.6565s
Protobuf: 1.2503s (112% faster)

### 1.000.000 Entries

Took too long

# Deserialize

## Small

JSON: 354.5 micro secs
Protobuf: 87.5830 micro secs (304% faster)

## Mid

JSON: 487.2920 micro secs
Protobuf: 106.8750 micro secs (355% faster)

## Large

JSON: 4.6815ms
Protobuf: 1.7539ms (166% faster)

# File size

## Small

JSON: 116 bytes
Protobuf: 66 bytes (43% smaller)

## Mid

JSON: 351 bytes
Protobuf: 230 bytes (34% smaller)

## Large

JSON: 113072 bytes
Protobuf: 59945 bytes (47% smaller)
