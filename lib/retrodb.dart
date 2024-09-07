library retrodb;

import 'dart:io';
import 'dart:typed_data';
import 'package:path_provider/path_provider.dart';
import 'package:sqflite_common/sqlite_api.dart';
import 'models.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:path/path.dart' as path;

export "./models.dart";

class RetroDatabase {
  final Database db;
  RetroDatabase(this.db);

  static Future<RetroDatabase> open(DatabaseFactory factory, File? file) async {
    if (file == null) {
      final bytes = Uint8List.sublistView(
          await rootBundle.load("libretrodb-sqlite/build/libretrodb.sqlite"));

      final dir = await getApplicationDocumentsDirectory();
      file = File(path.join(dir.path, "libretrodb.sqlite"));

      var exists = false;
      try {
        await file.create(exclusive: true);
      } on PathExistsException {
        exists = true;
      } catch (e) {
        rethrow;
      }

      if (!exists) await factory.writeDatabaseBytes(file.path, bytes);
    }

    return RetroDatabase(await factory.openDatabase(file.path,
        options: OpenDatabaseOptions(readOnly: true)));
  }

  Future<Iterable<Game>> query({
    bool? distinct,
    List<Column>? columns,
    String? where,
    List<Object?>? whereArgs,
    // String? groupBy,
    // String? having,
    // List<(Column, Order)>? orderBy,
    int? limit,
    int? offset,
  }) async {
    columns ??= [
      Column.serialId,
      Column.developer,
      Column.publisher,
      Column.rating,
      Column.users,
      Column.franchise,
      Column.releaseYear,
      Column.releaseMonth,
      Column.region,
      Column.genre,
      Column.displayName,
      Column.fullName,
      Column.platform,
      Column.roms,
    ];

    List<String> joins = List.empty(growable: true);
    final rawColumns = columns
        .expand((col) => transformColumn(col, joins))
        .followedBy(["games.id"]).join(",");
    final rawJoins = joins.join(" ");

    final rawLimit = limit != null ? " LIMIT $limit" : null;
    final rawOffset = offset != null ? " OFFSET $offset" : null;

    final sql =
        "SELECT $rawColumns FROM games $rawJoins WHERE $where${rawLimit ?? ""}${rawOffset ?? ""}";

    final cursor = await db.rawQueryCursor(sql, whereArgs);
    final games = <int, Game>{};
    try {
      final includeRoms = columns.contains(Column.roms);

      while (await cursor.moveNext()) {
        final row = cursor.current;
        final rom = includeRoms ? Rom.fromJson(row) : null;

        games.update(row["id"] as int, (game) {
          if (rom != null) game.roms?.add(rom);
          return game;
        },
            ifAbsent: () =>
                Game.fromJson(row)..roms = includeRoms ? [rom!] : null);
      }
    } finally {
      await cursor.close();
    }

    return games.values;
  }

  Future<void> close() async {
    await db.close();
  }
}

enum Column {
  serialId,
  developer,
  publisher,
  rating,
  users,
  franchise,
  releaseYear,
  releaseMonth,
  region,
  genre,
  displayName,
  fullName,
  platform,
  roms,
}

List<String> transformColumn(Column col, List<String> joins) {
  switch (col) {
    case Column.serialId:
      return ["games.serial_id"];
    case Column.developer:
      joins.add("LEFT JOIN developers ON games.developer_id = developers.id");
      return ["developers.name AS developer"];
    case Column.publisher:
      joins.add("LEFT JOIN publishers ON games.publisher_id = publishers.id");
      return ["publishers.name AS publisher"];
    case Column.rating:
      joins.add("LEFT JOIN ratings ON games.rating_id = ratings.id");
      return ["ratings.name AS rating"];
    case Column.users:
      return ["games.users"];
    case Column.franchise:
      joins.add("LEFT JOIN franchises ON games.franchise_id = franchises.id");
      return ["franchises.name AS franchise"];
    case Column.releaseYear:
      return ["games.release_year"];
    case Column.releaseMonth:
      return ["games.release_month"];
    case Column.region:
      joins.add("LEFT JOIN regions ON games.region_id = regions.id");
      return ["regions.name AS region"];
    case Column.genre:
      joins.add("LEFT JOIN genres ON games.genre_id = genres.id");
      return ["genres.name AS genre"];
    case Column.displayName:
      return ["games.display_name"];
    case Column.fullName:
      return ["games.full_name"];
    case Column.platform:
      joins.addAll([
        "LEFT JOIN platforms ON games.platform_id = platforms.id",
        "LEFT JOIN manufacturers ON manufacturers.id = platforms.manufacturer_id"
      ]);
      return ["platforms.name AS platform, manufacturers.name AS manufacturer"];
    case Column.roms:
      joins.add("INNER JOIN roms ON games.serial_id = roms.serial_id");
      return [
        "roms.id as rom_id",
        "roms.serial_id as rom_serial_id",
        "roms.name as rom_name",
        "roms.crc as rom_crc"
      ];
  }
}

enum Order { ascending, descending }

String orderToString(Order ord) {
  return switch (ord) {
    Order.ascending => "ASC",
    Order.descending => "DESC",
  };
}
