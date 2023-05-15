# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [v0.5.0] - 2023-05-15

### Added

- `time`featureを追加

### Fixed

- ペイロードプロパティの漏れを修正

## [v0.4.1] - 2023-05-12

### Added

- [Codecov](https://codecov.io)レポートを追加
    - https://app.codecov.io/gh/H1rono/traq-bot-http-rs

### Changed

- テストを追加
- ドキュメントを修正

## [v0.4.0] - 2023-03-11

### Added

- `uuid`featureを追加

## [v0.3.1] - 2023-03-09

### Fixed

- `payloads::UserCreatedPayload`と`payloads::types::MessageStamp`のフィールドがプライベートになっていたのを修正

## [v0.3.0] - 2023-03-06

### Added

- `ParseError`に`PartialEq`と`Eq`の実装を追加
- ドキュメンテーションコメントを追加

### Changed

- `RequestParser`のリファクタリング
- テストを充実

## [v0.2.0] - 2023-02-23

### Added

- `RequestParser`に`Debug`と`Clone`の実装を追加
- exampleにREADMEの例を実装

### Fixed

- Exampleがビルドできない問題を修正

## [v0.1.0] - 2023-02-22

- initial release

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

<!-- Versions -->
[v0.5.0]: https://github.com/H1rono/traq-bot-http/rs/compare/v0.4.1..v0.5.0
[v0.4.1]: https://github.com/H1rono/traq-bot-http/rs/compare/v0.4.0..v0.4.1
[v0.4.0]: https://github.com/H1rono/traq-bot-http/rs/compare/v0.3.1..v0.4.0
[v0.3.1]: https://github.com/H1rono/traq-bot-http/rs/compare/v0.3.0..v0.3.1
[v0.3.0]: https://github.com/H1rono/traq-bot-http/rs/compare/v0.2.0..v0.3.0
[v0.2.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.1.0..v0.2.0
[v0.1.0]: https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.0.1