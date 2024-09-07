library retrodb;

import 'dart:io';
import 'package:sqflite_common/sqlite_api.dart';

export "./models.dart";

class RetroDatabase {
  final Database db;
  RetroDatabase(this.db);

  Future<List<Map<String, Object?>>> query({
    bool? distinct,
    List<Column>? columns,
    String? where,
    List<Object?>? whereArgs,
    String? groupBy,
    String? having,
    List<(Column, Order)>? orderBy,
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

    final sql = "SELECT $rawColumns FROM games $rawJoins WHERE $where;";
    return await db.rawQuery(sql, whereArgs);
  }

  Future<void> close() async {
    await db.close();
  }

  static Future<RetroDatabase> open(DatabaseFactory factory, File file) async {
    return RetroDatabase(await factory.openDatabase(file.path,
        options: OpenDatabaseOptions(readOnly: true)));
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
      joins.add("LEFT JOIN platforms ON games.platform_id = platforms.id");
      return ["platforms.name AS platform"];
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
