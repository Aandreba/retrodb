import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:retrodb/retrodb.dart';
import 'package:sqflite_common_ffi/sqflite_ffi.dart';

void main() {
  sqfliteFfiInit();
  test('adds one to input values', () async {
    final db = await RetroDatabase.open(databaseFactoryFfi,
        File("/home/aandreba/libretrodb-sqlite/build/libretrodb.sqlite"));

    final search =
        await db.query(where: "rom_crc = ?", whereArgs: [3564459454]);

    // 0xD47555BE
    for (final res in search) {
      print(res.roms?.length);
    }

    // final calculator = Calculator();
    // expect(calculator.addOne(2), 3);
    // expect(calculator.addOne(-7), -6);
    // expect(calculator.addOne(0), 1);
  });
}
