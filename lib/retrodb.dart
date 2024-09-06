library retrodb;

import 'dart:io';
import 'package:sqflite_common/sqlite_api.dart';

/// A Calculator.
class RetroDatabase {
  final Database db;
  RetroDatabase(this.db);

  Future<void> close() async {
    await db.close();
  }

  static Future<RetroDatabase> open(DatabaseFactory factory, File file) async {
    return RetroDatabase(await factory.openDatabase(file.path,
        options: OpenDatabaseOptions(readOnly: true)));
  }
}
