# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [v0.11.2] - 2024-12-24

### Added

- Release assetsに[`cargo package`](https://doc.rust-lang.org/cargo/commands/cargo-package.html)の出力を追加

### Changed

- Cargoの不要な依存関係を削除
- `handler::WithState`の内部構造を修正

## [v0.11.1] - 2024-12-12

### Added

- 一部関数に`#[inline]`のヒントを追加

### Changed

- `RequestParser`の内部構造を修正
- cargoの依存関係を更新

### Fixed

- `ParseRequest`の内部実装を修正

## [v0.11.0] - 2024-12-09

### Added

- MSRVに関するドキュメントを追加

### Changed

- `Handler::on_{event_kind}`のシグネチャを修正
- MSRVを1.76.0に更新
- 依存ライブラリを更新
- `docs.rs`用の設定を修正
- `tower::Service` traitの実装条件を修正

### Removed

- `Handler::new`を削除
- `handler::EventWithState`を削除

### Fixed

- `<handler::WithState as tower::Service>::poll_ready`の実装を修正

### For Developers

- `#![deny(clippy::pedantic)]`の設定を`Cargo.toml`に移動
- CIの設定を一部修正

## [v0.10.2] - 2024-12-08

### Added

- READMEに`tower`featureに関する記述を追加
- `Handler`まわりのドキュメントを追加

### Changed

- `Future`型を具体的なものに変更
    - `RequestParser::parse_request`, `<Handler as Service>::Future`のシグネチャが変わりました
- `impl Service for Handler`を満たす条件が緩くなりました

### For Developers

- Cargo.tomlの`docs.rs`用設定を修正
- GitHub Actionsの依存関係を更新

## [v0.10.1] - 2024-11-09

### Added

- `Handler` APIを追加
- `Handler` を使用したexample `handler-with-axum` を追加

### For Developers

- GitHub Actionsの依存関係を更新

## [v0.10.0] - 2024-10-13

### Added

- `http`featureを追加
- `RequestParser::parse_request`を追加
    - `http`featureを有効にすると使用可能になります
- エラー型の変更に伴って`Error`型, `ErrorKind`型, `Result`型エイリアスを追加

### Changed

- `RequestParser::parse`などで返されるエラー型を変更
    - `ParseError`型から`Error`型へ
- axumを使用したexampleコードを, `http`featureを利用したものに変更

### Removed

- エラー型の変更に伴って`ParseError`型を削除

### For Developers

- GitHub ActionsからNixへの依存を削除
- dev-dependenciesに`futures`crateを追加
- GitHub Actions, Nix Flakesの依存関係を更新

## [v0.9.1] - 2024-09-14

### Added

- (internal) 内部で使用されているマクロにドキュメントを追加

### Changed

- `traq_bot_http::Event`および`traq_bot_http::EventKind`のドキュメントを一部修正
- (internal) ボイラープレート部分を新たに追加したマクロで置き換え

### For Developers

- Nix Flakesの依存関係を更新

## [v0.9.0] - 2024-07-09

### Added

- [rocket](https://rocket.rs)を使用したexampleを追加

### Changed

- 依存ライブラリを更新
- `RequestParser::parse_headers`および`RequestParser::parse`の引数を変更
    - `header`が`IntoIterator`を受け入れるようになりました
    - この変更でコンパイルできなくなるコードが存在する可能性があります
- 内部コードの改善
- [`clippy::cargo`](https://doc.rust-lang.org/clippy/lints.html#cargo)に対応

### For Developers

- Cargoの依存関係をDependabotの管理対象から削除
- GitHub Actions, Nix Flakesの依存関係を更新

## [v0.8.3] - 2024-04-13

### Added

- `Event`, `RequestParser`, `ParseError`型に`#[must_use]`を追加

### Changed

- 依存ライブラリを更新
- ドキュメント内の`.unwrap()`を全て`?`に変更 ([C-QUESTION-MARK](https://rust-lang.github.io/api-guidelines/documentation.html#c-question-mark))
- [`clippy::pedantic`](https://doc.rust-lang.org/clippy/lints.html#pedantic)に完全対応

### For Developers

- GitHub Actions, Nix Flakesの依存関係を更新

## [v0.8.2] - 2024-02-11

### Added

- `RequestParser::parse_headers`, `RequestParser::parse`のドキュメントを追加

### Changed

- rustfmtの設定を修正

### Fixed

- READMEの`Cargo.toml`記述例を修正
- `clippy::pedantic`の対象ルールに一部対応

### For Developers

- GitHub ActionsでのTOMLファイルのバリデーションを削除
- GitHub Actionsの依存関係を更新

## [v0.8.1] - 2023-11-30

### Changed

- `RequestParser::parse`のExampleを修正
- exampleに使用するライブラリのバージョンを更新

### For Developers

- 一部CIを修正
- flake.nixを改善
- CIの依存関係を更新

## [v0.8.0] - 2023-11-30

### Added

- `RequestParser::parse_headers`を公開
- (for developers) `rust-toolchain.toml`を追加
- (for developers) Nix flake, direnvをセットアップ
- (for developers) cSpellによるスペルチェックを追加

### Changed

- ペイロード型内の`type_`を`r#type`に変更
- `RequestParser::parse`の引数`headers`の型を`http::HeaderMap`からイテレータ型に変更
- `RequestParser::parse`の変更に伴ってexampleを修正
- MSRVを1.64.0から1.67.1に変更

### Removed

- `payloads::serde`を非公開に
- `http`クレートへの依存を削除

### Security

- cargo-audit, cargo-udepsのチェックを追加

## [v0.7.2] - 2023-10-13

### Added

- enum `EventKind`を追加

### Changed

- 一部ドキュメントを修正
- 内部実装、テストでマクロを使用

### Fixed

- `USER_GROUP_*`のイベントがパースできなかった問題を修正


## [v0.7.1] - 2023-09-25

### Fixed

- 外部に公開されていたマクロをプライベートに


## [v0.7.0] - 2023-09-24

### Added

- 以下のBOTイベントを追加
    - `USER_GROUP_CREATED`
    - `USER_GROUP_UPDATED`
    - `USER_GROUP_DELETED`
    - `USER_GROUP_MEMBER_ADDED`
    - `USER_GROUP_MEMBER_UPDATED`
    - `USER_GROUP_MEMBER_REMOVED`
    - `USER_GROUP_ADMIN_ADDED`
    - `USER_GROUP_ADMIN_REMOVED`

### Changed

- enum `Event`を[`non-exhaustive`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)に
- 型変換の実装にマクロを使用

### Fixed

- CI修正
- Codecovの設定を修正

## [v0.6.3] - 2023-09-02

### Added

- `From`によるペイロード間の型変換を追加
- 全てのペイロード型に`Into<Event>`, `FromStr`, `Display`を実装

### Changed

- テストコードの場所を変更

## [v0.6.2] - 2023-07-29

### Added

- ドキュメントにバッジを追加
- `Cargo.toml`に追記

### Removed

- テスト内のヘルパー関数を削除

## [v0.6.1] - 2023-07-26

### Changed

- Codecovの設定を更新
- リリース手順を修正

## [v0.6.0] - 2023-06-23

### Added

- `chrono`featureを追加

### Changed

- `payloads::serde::time`モジュールを`payloads::serde::timestamp`にrename

## [v0.5.2] - 2023-06-20

### Added

- 型エイリアス`payloads::types::{TimeStamp, Uuid}`を追加
- 型エイリアスに対応する`serialize`, `deserialize`関数群として`payloads::serde::{time, uuid}`を追加
- パッケージの`dev-dependencies`にexamplesで使用しているものを追加

### Changed

- examplesの構造を修正
- テストを追加した型エイリアスを用いたものに修正

## [v0.5.1] - 2023-05-20

### Added

- `StampCreatedPayload`のテストを追加

### Fixed

- `time`featureで`StampCreatedPayload::event_time`がプライベートになる問題を修正

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
[v0.11.2]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.11.1..v0.11.2
[v0.11.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.11.0..v0.11.1
[v0.11.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.10.2..v0.11.0
[v0.10.2]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.10.1..v0.10.2
[v0.10.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.10.0..v0.10.1
[v0.10.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.9.1..v0.10.0
[v0.9.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.9.0..v0.9.1
[v0.9.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.8.3..v0.9.0
[v0.8.3]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.8.2..v0.8.3
[v0.8.2]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.8.1..v0.8.2
[v0.8.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.8.0..v0.8.1
[v0.8.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.7.2..v0.8.0
[v0.7.2]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.7.1..v0.7.2
[v0.7.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.7.0..v0.7.1
[v0.7.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.6.3..v0.7.0
[v0.6.3]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.6.2..v0.6.3
[v0.6.2]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.6.1..v0.6.2
[v0.6.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.6.0..v0.6.1
[v0.6.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.5.2..v0.6.0
[v0.5.2]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.5.1..v0.5.2
[v0.5.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.5.0..v0.5.1
[v0.5.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.4.1..v0.5.0
[v0.4.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.4.0..v0.4.1
[v0.4.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.3.1..v0.4.0
[v0.3.1]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.3.0..v0.3.1
[v0.3.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.2.0..v0.3.0
[v0.2.0]: https://github.com/H1rono/traq-bot-http-rs/compare/v0.1.0..v0.2.0
[v0.1.0]: https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.0.1
