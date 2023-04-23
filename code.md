# My Code

```dart
// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility in the flutter_test package. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:sleepease/main.dart';

void main() {
  testWidgets('Counter increments smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const MyApp());

    // Verify that our counter starts at 0.
    expect(find.text('0'), findsOneWidget);
    expect(find.text('1'), findsNothing);

    // Tap the '+' icon and trigger a frame.
    await tester.tap(find.byIcon(Icons.add));
    await tester.pump();

    // Verify that our counter has incremented.
    expect(find.text('0'), findsNothing);
    expect(find.text('1'), findsOneWidget);
  });
}
```

```dart
import 'package:flutter_driver/flutter_driver.dart';
import 'package:test/test.dart';

void main() {
  group('Secure Storage Example', () {
    late HomePageObject pageObject;
    late FlutterDriver driver;

    // Connect to the Flutter driver before running any tests.
    setUpAll(() async {
      driver = await FlutterDriver.connect();
      pageObject = HomePageObject(driver);

      await pageObject.deleteAll();
    });

    // Close the connection to the driver after the tests have completed.
    tearDownAll(() async {
      await pageObject.deleteAll();
      driver.close();
    });

    test('basic operations', () async {
      await pageObject.hasNoRow(0);

      await pageObject.addRandom();
      await pageObject.hasRow(0);
      await pageObject.addRandom();
      await pageObject.hasRow(1);

      await pageObject.editRow('Row 0', 0);
      await pageObject.editRow('Row 1', 1);

      await Future.delayed(const Duration(seconds: 3));

      await pageObject.rowHasTitle('Row 0', 0);
      await pageObject.rowHasTitle('Row 1', 1);

      await Future.delayed(const Duration(seconds: 3));

      await pageObject.deleteRow(1);
      await pageObject.hasNoRow(1);

      await Future.delayed(const Duration(seconds: 3));

      await pageObject.rowHasTitle('Row 0', 0);
      await pageObject.deleteRow(0);
      await pageObject.hasNoRow(0);
    });
  });
}

class HomePageObject {
  HomePageObject(this.driver);

  final FlutterDriver driver;
  final _addRandomButtonFinder = find.byValueKey('add_random');
  final _deleteAllButtonFinder = find.byValueKey('delete_all');
  final _popUpMenuButtonFinder = find.byValueKey('popup_menu');

  Future deleteAll() async {
    await driver.tap(_popUpMenuButtonFinder);
    await driver.tap(_deleteAllButtonFinder);
  }

  Future addRandom() async {
    await driver.tap(_addRandomButtonFinder);
  }

  Future editRow(String title, int index) async {
    await driver.tap(find.byValueKey('popup_row_$index'));
    await driver.tap(find.byValueKey('edit_row_$index'));

    await driver.tap(find.byValueKey('title_field'));
    await driver.enterText(title);
    await driver.tap(find.byValueKey('save'));
  }

  Future rowHasTitle(String title, int index) async {
    expect(await driver.getText(find.byValueKey('title_row_$index')), title);
  }

  Future hasRow(int index) async {
    await driver.waitFor(find.byValueKey('title_row_$index'));
  }

  Future deleteRow(int index) async {
    await driver.tap(find.byValueKey('popup_row_$index'));
    await driver.tap(find.byValueKey('delete_row_$index'));
  }

  Future hasNoRow(int index) async {
    await driver.waitForAbsent(find.byValueKey('title_row_$index'));
  }
}
```

```dart
import 'package:flutter_driver/driver_extension.dart';
import 'package:flutter_secure_storage_example/main.dart' as app;

void main() {
  // This line enables the extension.
  enableFlutterDriverExtension();

  // Call the `main()` function of the app, or call `runApp` with
  // any widget you are interested in testing.
  app.main();
}
```

```dart
// This is a basic Flutter widget test.
// To perform an interaction with a widget in your test, use the WidgetTester utility that Flutter
// provides. For example, you can send tap and scroll gestures. You can also use WidgetTester to
// find child widgets in the widget tree, read text, and verify that the values of widget properties
// are correct.

void main() {}
```

```dart
import 'dart:async';
import 'dart:io';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

void main() {
  runApp(const MaterialApp(home: ItemsWidget()));
}

class ItemsWidget extends StatefulWidget {
  const ItemsWidget({Key? key}) : super(key: key);

  @override
  _ItemsWidgetState createState() => _ItemsWidgetState();
}

enum _Actions { deleteAll }
enum _ItemActions { delete, edit, containsKey }

class _ItemsWidgetState extends State<ItemsWidget> {
  final _storage = const FlutterSecureStorage();
  final _accountNameController =
      TextEditingController(text: 'flutter_secure_storage_service');

  List<_SecItem> _items = [];

  @override
  void initState() {
    super.initState();

    _accountNameController.addListener(() => _readAll());
    _readAll();
  }

  Future<void> _readAll() async {
    final all = await _storage.readAll(
        iOptions: _getIOSOptions(), aOptions: _getAndroidOptions());
    setState(() {
      _items = all.entries
          .map((entry) => _SecItem(entry.key, entry.value))
          .toList(growable: false);
    });
  }

  void _deleteAll() async {
    await _storage.deleteAll(
        iOptions: _getIOSOptions(), aOptions: _getAndroidOptions());
    _readAll();
  }

  void _addNewItem() async {
    final String key = _randomValue();
    final String value = _randomValue();

    await _storage.write(
        key: key,
        value: value,
        iOptions: _getIOSOptions(),
        aOptions: _getAndroidOptions());
    _readAll();
  }

  IOSOptions _getIOSOptions() => IOSOptions(
        accountName: _getAccountName(),
      );

  AndroidOptions _getAndroidOptions() => const AndroidOptions(
        encryptedSharedPreferences: true,
      );

  String? _getAccountName() =>
      _accountNameController.text.isEmpty ? null : _accountNameController.text;

  @override
  Widget build(BuildContext context) => Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
          actions: <Widget>[
            IconButton(
                key: const Key('add_random'),
                onPressed: _addNewItem,
                icon: const Icon(Icons.add)),
            PopupMenuButton<_Actions>(
                key: const Key('popup_menu'),
                onSelected: (action) {
                  switch (action) {
                    case _Actions.deleteAll:
                      _deleteAll();
                      break;
                  }
                },
                itemBuilder: (BuildContext context) =>
                    <PopupMenuEntry<_Actions>>[
                      const PopupMenuItem(
                        key: Key('delete_all'),
                        value: _Actions.deleteAll,
                        child: Text('Delete all'),
                      ),
                    ])
          ],
        ),
        body: Column(
          children: [
            if (!kIsWeb && Platform.isIOS)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: TextFormField(
                  controller: _accountNameController,
                  decoration:
                      const InputDecoration(labelText: 'kSecAttrService'),
                ),
              ),
            Expanded(
              child: ListView.builder(
                itemCount: _items.length,
                itemBuilder: (BuildContext context, int index) => ListTile(
                  trailing: PopupMenuButton(
                      key: Key('popup_row_$index'),
                      onSelected: (_ItemActions action) =>
                          _performAction(action, _items[index], context),
                      itemBuilder: (BuildContext context) =>
                          <PopupMenuEntry<_ItemActions>>[
                            PopupMenuItem(
                              value: _ItemActions.delete,
                              child: Text(
                                'Delete',
                                key: Key('delete_row_$index'),
                              ),
                            ),
                            PopupMenuItem(
                              value: _ItemActions.edit,
                              child: Text(
                                'Edit',
                                key: Key('edit_row_$index'),
                              ),
                            ),
                            PopupMenuItem(
                              value: _ItemActions.containsKey,
                              child: Text(
                                'Contains Key',
                                key: Key('contains_row_$index'),
                              ),
                            ),
                          ]),
                  title: Text(
                    _items[index].value,
                    key: Key('title_row_$index'),
                  ),
                  subtitle: Text(
                    _items[index].key,
                    key: Key('subtitle_row_$index'),
                  ),
                ),
              ),
            ),
          ],
        ),
      );

  Future<void> _performAction(
      _ItemActions action, _SecItem item, BuildContext context) async {
    switch (action) {
      case _ItemActions.delete:
        await _storage.delete(
            key: item.key,
            iOptions: _getIOSOptions(),
            aOptions: _getAndroidOptions());
        _readAll();

        break;
      case _ItemActions.edit:
        final result = await showDialog<String>(
            context: context,
            builder: (context) => _EditItemWidget(item.value));
        if (result != null) {
          await _storage.write(
              key: item.key,
              value: result,
              iOptions: _getIOSOptions(),
              aOptions: _getAndroidOptions());
          _readAll();
        }
        break;
      case _ItemActions.containsKey:
        final result = await _storage.containsKey(key: item.key);
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(
          content: Text('Contains Key: $result'),
          // backgroundColor: Colors.green,
          duration: const Duration(seconds: 4),
        ));
        break;
    }
  }

  String _randomValue() {
    final rand = Random();
    final codeUnits = List.generate(20, (index) {
      return rand.nextInt(26) + 65;
    });

    return String.fromCharCodes(codeUnits);
  }
}

class _EditItemWidget extends StatelessWidget {
  _EditItemWidget(String text)
      : _controller = TextEditingController(text: text);

  final TextEditingController _controller;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Edit item'),
      content: TextField(
        key: const Key('title_field'),
        controller: _controller,
        autofocus: true,
      ),
      actions: <Widget>[
        TextButton(
            key: const Key('cancel'),
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel')),
        TextButton(
            key: const Key('save'),
            onPressed: () => Navigator.of(context).pop(_controller.text),
            child: const Text('Save')),
      ],
    );
  }
}

class _SecItem {
  _SecItem(this.key, this.value);

  final String key;
  final String value;
}
```

```dart
part of flutter_secure_storage;

class WebOptions extends Options {
  const WebOptions({
    this.dbName = 'FlutterEncryptedStorage',
    this.publicKey = 'FlutterSecureStorage',
  });

  static const WebOptions defaultOptions = WebOptions();

  final String dbName;
  final String publicKey;

  @override
  Map<String, String> toMap() => <String, String>{
        'dbName': dbName,
        'publicKey': publicKey,
      };
}
```

```dart
part of flutter_secure_storage;

// KeyChain accessibility attributes as defined here:
// https://developer.apple.com/documentation/security/ksecattraccessible?language=objc
enum IOSAccessibility {
  // The data in the keychain can only be accessed when the device is unlocked.
  // Only available if a passcode is set on the device.
  // Items with this attribute do not migrate to a new device.
  passcode,

  // The data in the keychain item can be accessed only while the device is unlocked by the user.
  unlocked,

  // The data in the keychain item can be accessed only while the device is unlocked by the user.
  // Items with this attribute do not migrate to a new device.
  unlocked_this_device,

  // The data in the keychain item cannot be accessed after a restart until the device has been unlocked once by the user.
  first_unlock,

  // The data in the keychain item cannot be accessed after a restart until the device has been unlocked once by the user.
  // Items with this attribute do not migrate to a new device.
  first_unlock_this_device,
}

class IOSOptions extends Options {
  const IOSOptions({
    String? groupId,
    String? accountName = IOSOptions.defaultAccountName,
    IOSAccessibility accessibility = IOSAccessibility.unlocked,
    bool synchronizable = false,
  })  : _groupId = groupId,
        _accessibility = accessibility,
        _accountName = accountName,
        _synchronizable = synchronizable;

  static const defaultAccountName = 'flutter_secure_storage_service';

  static const IOSOptions defaultOptions = IOSOptions();

  final String? _groupId;
  final String? _accountName;
  final IOSAccessibility _accessibility;
  final bool _synchronizable;

  @override
  Map<String, String> toMap() => <String, String>{
        'accessibility': describeEnum(_accessibility),
        if (_accountName != null) 'accountName': _accountName!,
        if (_groupId != null) 'groupId': _groupId!,
        'synchronizable': '$_synchronizable',
      };

  IOSOptions copyWith({
    String? groupId,
    String? accountName,
    IOSAccessibility? accessibility,
    bool? synchronizable,
  }) =>
      IOSOptions(
        groupId: groupId ?? _groupId,
        accountName: accountName ?? _accountName,
        accessibility: accessibility ?? _accessibility,
        synchronizable: synchronizable ?? _synchronizable,
      );
}
```

```dart
part of flutter_secure_storage;

class MacOsOptions extends Options {
  const MacOsOptions();

  static const MacOsOptions defaultOptions = MacOsOptions();

  @override
  Map<String, String> toMap() => <String, String>{};
}
```

```dart
part of flutter_secure_storage;

class AndroidOptions extends Options {
  const AndroidOptions(
      {bool encryptedSharedPreferences = false, bool resetOnError = false})
      : _encryptedSharedPreferences = encryptedSharedPreferences,
        _resetOnError = resetOnError;

  /// EncryptedSharedPrefences are only available on API 23 and greater
  final bool _encryptedSharedPreferences;

  /// When an error is detected, automatically reset all data. This will prevent
  /// fatal errors regarding an unknown key however keep in mind that it will
  /// PERMANENLTY erase the data when an error occurs.
  ///
  /// Defaults to false.
  final bool _resetOnError;

  static const AndroidOptions defaultOptions = AndroidOptions();

  @override
  Map<String, String> toMap() => <String, String>{
        'encryptedSharedPreferences': '$_encryptedSharedPreferences',
        'resetOnError': '$_resetOnError'
      };

  AndroidOptions copyWith(
          {bool? encryptedSharedPreferences, bool? resetOnError}) =>
      AndroidOptions(
          encryptedSharedPreferences:
              encryptedSharedPreferences ?? _encryptedSharedPreferences,
          resetOnError: resetOnError ?? _resetOnError);
}
```

```dart
part of flutter_secure_storage;

class LinuxOptions extends Options {
  const LinuxOptions();

  static const LinuxOptions defaultOptions = LinuxOptions();

  @override
  Map<String, String> toMap() {
    return <String, String>{};
  }
}
```

```dart
part of flutter_secure_storage;

class WindowsOptions extends Options {
  const WindowsOptions();

  static const WindowsOptions defaultOptions = WindowsOptions();

  @override
  Map<String, String> toMap() => <String, String>{};
}
```

```dart
library flutter_secure_storage;

import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter_secure_storage_platform_interface/flutter_secure_storage_platform_interface.dart';

part './options/web_options.dart';
part './options/ios_options.dart';
part './options/android_options.dart';
part './options/linux_options.dart';
part './options/windows_options.dart';
part './options/macos_options.dart';

class FlutterSecureStorage {
  final IOSOptions iOptions;
  final AndroidOptions aOptions;
  final LinuxOptions lOptions;
  final WindowsOptions wOptions;
  final WebOptions webOptions;
  final MacOsOptions mOptions;

  const FlutterSecureStorage({
    this.iOptions = IOSOptions.defaultOptions,
    this.aOptions = AndroidOptions.defaultOptions,
    this.lOptions = LinuxOptions.defaultOptions,
    this.wOptions = WindowsOptions.defaultOptions,
    this.webOptions = WebOptions.defaultOptions,
    this.mOptions = MacOsOptions.defaultOptions,
  });

  static const UNSUPPORTED_PLATFORM = 'unsupported_platform';
  static final _platform = FlutterSecureStoragePlatform.instance;

  /// Encrypts and saves the [key] with the given [value].
  ///
  /// If the key was already in the storage, its associated value is changed.
  /// If the value is null, deletes associated value for the given [key].
  /// [key] shouldn't be null.
  /// [value] required value
  /// [iOptions] optional iOS options
  /// [aOptions] optional Android options
  /// [lOptions] optional Linux options
  /// [webOptions] optional web options
  /// [mOptions] optional MacOs options
  /// [wOptions] optional Windows options
  /// Can throw a [PlatformException].
  Future<void> write({
    required String key,
    required String? value,
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  }) =>
      value == null
          ? _platform.delete(
              key: key,
              options: _selectOptions(
                  iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
            )
          : _platform.write(
              key: key,
              value: value,
              options: _selectOptions(
                  iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
            );

  /// Decrypts and returns the value for the given [key] or null if [key] is not in the storage.
  ///
  /// [key] shouldn't be null.
  /// [iOptions] optional iOS options
  /// [aOptions] optional Android options
  /// [lOptions] optional Linux options
  /// [webOptions] optional web options
  /// [mOptions] optional MacOs options
  /// [wOptions] optional Windows options
  /// Can throw a [PlatformException].
  Future<String?> read({
    required String key,
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  }) =>
      _platform.read(
        key: key,
        options: _selectOptions(
            iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
      );

  /// Returns true if the storage contains the given [key].
  ///
  /// [key] shouldn't be null.
  /// [iOptions] optional iOS options
  /// [aOptions] optional Android options
  /// [lOptions] optional Linux options
  /// [webOptions] optional web options
  /// [mOptions] optional MacOs options
  /// [wOptions] optional Windows options
  /// Can throw a [PlatformException].
  Future<bool> containsKey({
    required String key,
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  }) =>
      _platform.containsKey(
        key: key,
        options: _selectOptions(
            iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
      );

  /// Deletes associated value for the given [key].
  ///
  /// [key] shouldn't be null.
  /// [iOptions] optional iOS options
  /// [aOptions] optional Android options
  /// [lOptions] optional Linux options
  /// [webOptions] optional web options
  /// [mOptions] optional MacOs options
  /// [wOptions] optional Windows options
  /// Can throw a [PlatformException].
  Future<void> delete({
    required String key,
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  }) =>
      _platform.delete(
        key: key,
        options: _selectOptions(
            iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
      );

  /// Decrypts and returns all keys with associated values.
  ///
  /// [iOptions] optional iOS options
  /// [aOptions] optional Android options
  /// [lOptions] optional Linux options
  /// [webOptions] optional web options
  /// [mOptions] optional MacOs options
  /// [wOptions] optional Windows options
  /// Can throw a [PlatformException].
  Future<Map<String, String>> readAll({
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  }) =>
      _platform.readAll(
        options: _selectOptions(
            iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
      );

  /// Deletes all keys with associated values.
  ///
  /// [iOptions] optional iOS options
  /// [aOptions] optional Android options
  /// [lOptions] optional Linux options
  /// [webOptions] optional web options
  /// [mOptions] optional MacOs options
  /// [wOptions] optional Windows options
  /// Can throw a [PlatformException].
  Future<void> deleteAll({
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  }) =>
      _platform.deleteAll(
        options: _selectOptions(
            iOptions, aOptions, lOptions, webOptions, mOptions, wOptions),
      );

  /// Select correct options based on current platform
  Map<String, String> _selectOptions(
    IOSOptions? iOptions,
    AndroidOptions? aOptions,
    LinuxOptions? lOptions,
    WebOptions? webOptions,
    MacOsOptions? mOptions,
    WindowsOptions? wOptions,
  ) {
    if (kIsWeb) {
      return webOptions?.params ?? this.webOptions.params;
    } else if (Platform.isLinux) {
      return lOptions?.params ?? this.lOptions.params;
    } else if (Platform.isIOS) {
      return iOptions?.params ?? this.iOptions.params;
    } else if (Platform.isAndroid) {
      return aOptions?.params ?? this.aOptions.params;
    } else if (Platform.isWindows) {
      return wOptions?.params ?? this.wOptions.params;
    } else if (Platform.isMacOS) {
      return mOptions?.params ?? this.mOptions.params;
    } else {
      throw UnsupportedError(UNSUPPORTED_PLATFORM);
    }
  }
}
```

```dart
// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility that Flutter provides. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:fluttertoast_example/main.dart';

void main() {
  testWidgets('Verify Platform version', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(MyApp());

    // Verify that platform version is retrieved.
    expect(
      find.byWidgetPredicate(
        (Widget widget) => widget is Text &&
                           widget.data!.startsWith('Running on:'),
      ),
      findsOneWidget,
    );
  });
}
```

```dart
import 'dart:async';

import 'package:fluttertoast_example/main.dart';
import 'package:flutter/material.dart';
import 'package:fluttertoast/fluttertoast.dart';

class ToastContext extends StatefulWidget {
  @override
  _ToastContextState createState() => _ToastContextState();
}

class _ToastContextState extends State<ToastContext> {
  late FToast fToast;

  Widget toast = Container(
    padding: const EdgeInsets.symmetric(horizontal: 24.0, vertical: 12.0),
    decoration: BoxDecoration(
      borderRadius: BorderRadius.circular(25.0),
      color: Colors.greenAccent,
    ),
    child: Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(Icons.check),
        SizedBox(
          width: 12.0,
        ),
        Text("This is a Custom Toast"),
      ],
    ),
  );

  _showToast() {
    fToast.showToast(
      child: toast,
      gravity: ToastGravity.BOTTOM,
      toastDuration: Duration(seconds: 2),
    );
  }

  _showBuilderToast() {
    fToast.showToast(
        child: toast,
        gravity: ToastGravity.BOTTOM,
        toastDuration: Duration(seconds: 2),
        positionedToastBuilder: (context, child) {
          return Positioned(
            child: child,
            top: 16.0,
            left: 16.0,
          );
        });
  }

  _showToastCancel() {
    Widget toastWithButton = Container(
      padding: const EdgeInsets.symmetric(horizontal: 24.0, vertical: 12.0),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(25.0),
        color: Colors.redAccent,
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Expanded(
            child: Text(
              "This is a Custom Toast This is a Custom Toast This is a Custom Toast This is a Custom Toast This is a Custom Toast This is a Custom Toast",
              softWrap: true,
              style: TextStyle(
                color: Colors.white,
              ),
            ),
          ),
          IconButton(
            icon: Icon(
              Icons.close,
            ),
            color: Colors.white,
            onPressed: () {
              fToast.removeCustomToast();
            },
          )
        ],
      ),
    );
    fToast.showToast(
      child: toastWithButton,
      gravity: ToastGravity.CENTER,
      toastDuration: Duration(seconds: 50),
    );
  }

  _queueToasts() {
    fToast.showToast(
      child: toast,
      gravity: ToastGravity.CENTER,
      toastDuration: Duration(seconds: 2),
    );
    fToast.showToast(
      child: toast,
      gravity: ToastGravity.BOTTOM,
      toastDuration: Duration(seconds: 2),
    );
    fToast.showToast(
      child: toast,
      gravity: ToastGravity.TOP,
      toastDuration: Duration(seconds: 2),
    );
    fToast.showToast(
      child: toast,
      gravity: ToastGravity.CENTER,
      toastDuration: Duration(seconds: 2),
    );
    fToast.showToast(
      child: toast,
      gravity: ToastGravity.TOP,
      toastDuration: Duration(seconds: 2),
    );
  }

  _removeToast() {
    fToast.removeCustomToast();
  }

  _removeAllQueuedToasts() {
    fToast.removeQueuedCustomToasts();
  }

  @override
  void initState() {
    super.initState();
    fToast = FToast();
    fToast.init(globalKey.currentState!.context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Custom Toasts"),
      ),
      body: Column(
        children: [
          SizedBox(
            height: 24.0,
          ),
          ElevatedButton(
            child: Text("Show Custom Toast"),
            onPressed: () {
              _showToast();
            },
          ),
          ElevatedButton(
            child: Text("Show Custom Toast via PositionedToastBuilder"),
            onPressed: () {
              _showBuilderToast();
            },
          ),
          SizedBox(
            height: 24.0,
          ),
          ElevatedButton(
            child: Text("Custom Toast With Close Button"),
            onPressed: () {
              _showToastCancel();
            },
          ),
          SizedBox(
            height: 24.0,
          ),
          ElevatedButton(
            child: Text("Queue Toasts"),
            onPressed: () {
              _queueToasts();
            },
          ),
          SizedBox(
            height: 24.0,
          ),
          ElevatedButton(
            child: Text("Cancel Toast"),
            onPressed: () {
              _removeToast();
            },
          ),
          SizedBox(
            height: 24.0,
          ),
          ElevatedButton(
            child: Text("Remove Queued Toasts"),
            onPressed: () {
              _removeAllQueuedToasts();
            },
          ),
        ],
      ),
    );
  }
}
```

```dart
import 'package:flutter/material.dart';
import 'package:fluttertoast/fluttertoast.dart';

class ToastNoContext extends StatefulWidget {
  @override
  _ToastNoContextState createState() => _ToastNoContextState();
}

class _ToastNoContextState extends State<ToastNoContext> {
  void showLongToast() {
    Fluttertoast.showToast(
      msg: "This is Long Toast",
      toastLength: Toast.LENGTH_LONG,
      fontSize: 18.0,
    );
  }

  void showWebColoredToast() {
    Fluttertoast.showToast(
      msg: "This is Colored Toast with android duration of 5 Sec",
      toastLength: Toast.LENGTH_SHORT,
      webBgColor: "#e74c3c",
      textColor: Colors.black,
      timeInSecForIosWeb: 5,
    );
  }

  void showColoredToast() {
    Fluttertoast.showToast(
        msg: "This is Colored Toast with android duration of 5 Sec",
        toastLength: Toast.LENGTH_SHORT,
        backgroundColor: Colors.red,
        textColor: Colors.white);
  }

  void showShortToast() {
    Fluttertoast.showToast(
        msg: "This is Short Toast",
        toastLength: Toast.LENGTH_SHORT,
        timeInSecForIosWeb: 1);
  }

  void showTopShortToast() {
    Fluttertoast.showToast(
        msg: "This is Top Short Toast",
        toastLength: Toast.LENGTH_SHORT,
        gravity: ToastGravity.TOP,
        timeInSecForIosWeb: 1);
  }

  void showCenterShortToast() {
    Fluttertoast.showToast(
        msg: "This is Center Short Toast",
        toastLength: Toast.LENGTH_SHORT,
        gravity: ToastGravity.CENTER,
        timeInSecForIosWeb: 1);
  }

  void cancelToast() {
    Fluttertoast.cancel();
  }

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return new MaterialApp(
      home: new Scaffold(
        appBar: new AppBar(
          title: new Text('Flutter Toast'),
        ),
        body: new Center(
          child: new Column(
            children: <Widget>[
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                    child: new Text('Show Long Toast'),
                    onPressed: showLongToast),
              ),
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                    child: new Text('Show Short Toast'),
                    onPressed: showShortToast),
              ),
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                    child: new Text('Show Center Short Toast'),
                    onPressed: showCenterShortToast),
              ),
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                    child: new Text('Show Top Short Toast'),
                    onPressed: showTopShortToast),
              ),
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                    child: new Text('Show Colored Toast'),
                    onPressed: showColoredToast),
              ),
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                    child: new Text('Show  Web Colored Toast'),
                    onPressed: showWebColoredToast),
              ),
              new Padding(
                padding: const EdgeInsets.all(10.0),
                child: new ElevatedButton(
                  child: new Text('Cancel Toasts'),
                  onPressed: cancelToast,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
```

```dart
import 'package:fluttertoast/fluttertoast.dart';
import 'package:fluttertoast_example/toast_context.dart';
import 'package:fluttertoast_example/toast_no_context.dart';
import 'package:flutter/material.dart';

GlobalKey globalKey = GlobalKey();

void main() => runApp(
      MaterialApp(
        home: MyApp(),
      ),
    );

class MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      key: globalKey,
      appBar: AppBar(
        title: Text("Toast"),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          ElevatedButton(
            onPressed: () {
              Navigator.of(context).push(MaterialPageRoute(
                builder: (context) => ToastNoContext(),
              ));
            },
            child: Text("Flutter Toast No Context"),
          ),
          SizedBox(
            height: 24.0,
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.of(context).push(MaterialPageRoute(
                builder: (context) => ToastContext(),
              ));
            },
            child: Text("Flutter Toast Context"),
          ),
        ],
      ),
    );
  }
}
```

```dart
//
// Generated file. Do not edit.
//

// ignore_for_file: directives_ordering
// ignore_for_file: lines_longer_than_80_chars

import 'package:fluttertoast/fluttertoast_web.dart';

import 'package:flutter_web_plugins/flutter_web_plugins.dart';

// ignore: public_member_api_docs
void registerPlugins(Registrar registrar) {
  FluttertoastWebPlugin.registerWith(registrar);
  registrar.registerMessageHandler();
}
```

```dart
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

/// Toast Length
/// Only for Android Platform
enum Toast {
  /// Show Short toast for 1 sec
  LENGTH_SHORT,

  /// Show Long toast for 5 sec
  LENGTH_LONG
}

/// ToastGravity
/// Used to define the position of the Toast on the screen
enum ToastGravity {
  TOP,
  BOTTOM,
  CENTER,
  TOP_LEFT,
  TOP_RIGHT,
  BOTTOM_LEFT,
  BOTTOM_RIGHT,
  CENTER_LEFT,
  CENTER_RIGHT,
  SNACKBAR
}

/// Plugin to show a toast message on screen
/// Only for android, ios and Web platforms
class Fluttertoast {
  /// [MethodChannel] used to communicate with the platform side.
  static const MethodChannel _channel =
      const MethodChannel('PonnamKarthik/fluttertoast');

  /// Let say you have an active show
  /// Use this method to hide the toast immediately
  static Future<bool?> cancel() async {
    bool? res = await _channel.invokeMethod("cancel");
    return res;
  }

  /// Summons the platform's showToast which will display the message
  ///
  /// Wraps the platform's native Toast for android.
  /// Wraps the Plugin https://github.com/scalessec/Toast for iOS
  /// Wraps the https://github.com/apvarun/toastify-js for Web
  ///
  /// Parameter [msg] is required and all remaining are optional
  static Future<bool?> showToast({
    required String msg,
    Toast? toastLength,
    int timeInSecForIosWeb = 1,
    double? fontSize,
    ToastGravity? gravity,
    Color? backgroundColor,
    Color? textColor,
    bool webShowClose = false,
    webBgColor: "linear-gradient(to right, #00b09b, #96c93d)",
    webPosition: "right",
  }) async {
    String toast = "short";
    if (toastLength == Toast.LENGTH_LONG) {
      toast = "long";
    }

    String gravityToast = "bottom";
    if (gravity == ToastGravity.TOP) {
      gravityToast = "top";
    } else if (gravity == ToastGravity.CENTER) {
      gravityToast = "center";
    } else {
      gravityToast = "bottom";
    }

//lines from 78 to 97 have been changed in order to solve issue #328
    if (backgroundColor == null) {
      backgroundColor = Colors.black;
    }
    if (textColor == null) {
      textColor = Colors.white;
    }
    final Map<String, dynamic> params = <String, dynamic>{
      'msg': msg,
      'length': toast,
      'time': timeInSecForIosWeb,
      'gravity': gravityToast,
      'bgcolor': backgroundColor != null ? backgroundColor.value : null,
      'iosBgcolor': backgroundColor != null ? backgroundColor.value : null,
      'textcolor': textColor != null ? textColor.value : null,
      'iosTextcolor': textColor != null ? textColor.value : null,
      'fontSize': fontSize,
      'webShowClose': webShowClose,
      'webBgColor': webBgColor,
      'webPosition': webPosition
    };

    bool? res = await _channel.invokeMethod('showToast', params);
    return res;
  }
}

/// Signature for a function to buildCustom Toast
typedef PositionedToastBuilder = Widget Function(
    BuildContext context, Widget child);

/// Runs on dart side this has no interaction with the Native Side
/// Works with all platforms just in two lines of code
/// final fToast = FToast().init(context)
/// fToast.showToast(child)
///
class FToast {
  BuildContext? context;

  static final FToast _instance = FToast._internal();

  /// Prmary Constructor for FToast
  factory FToast() {
    return _instance;
  }

  /// Take users Context and saves to avariable
  FToast init(BuildContext context) {
    _instance.context = context;
    return _instance;
  }

  FToast._internal();

  OverlayEntry? _entry;
  List<_ToastEntry> _overlayQueue = [];
  Timer? _timer;

  /// Internal function which handles the adding
  /// the overlay to the screen
  ///
  _showOverlay() {
    if (_overlayQueue.length == 0) {
      _entry = null;
      return;
    }
    _ToastEntry _toastEntry = _overlayQueue.removeAt(0);
    _entry = _toastEntry.entry;
    if (context == null)
      throw ("Error: Context is null, Please call init(context) before showing toast.");
    Overlay.of(context!)?.insert(_entry!);

    _timer = Timer(_toastEntry.duration!, () {
      Future.delayed(Duration(milliseconds: 360), () {
        removeCustomToast();
      });
    });
  }

  /// If any active toast present
  /// call removeCustomToast to hide the toast immediately
  removeCustomToast() {
    _timer?.cancel();
    _timer = null;
    if (_entry != null) _entry!.remove();
    _entry = null;
    _showOverlay();
  }

  /// FToast maintains a queue for every toast
  /// if we called showToast for 3 times we all to queue
  /// and show them one after another
  ///
  /// call removeCustomToast to hide the toast immediately
  removeQueuedCustomToasts() {
    _timer?.cancel();
    _timer = null;
    _overlayQueue.clear();
    if (_entry != null) _entry!.remove();
    _entry = null;
  }

  /// showToast accepts all the required paramenters and prepares the child
  /// calls _showOverlay to display toast
  ///
  /// Paramenter [child] is requried
  /// fadeDuration default is 350 milliseconds
  void showToast({
    required Widget child,
    PositionedToastBuilder? positionedToastBuilder,
    Duration? toastDuration,
    ToastGravity? gravity,
    int fadeDuration = 350,
  }) {
    if (context == null)
      throw ("Error: Context is null, Please call init(context) before showing toast.");
    Widget newChild = _ToastStateFul(
        child, toastDuration ?? Duration(seconds: 2),
        fadeDuration: fadeDuration);

    /// Check for keyboard open
    /// If open will ignore the gravity bottom and change it to center
    if (gravity == ToastGravity.BOTTOM) {
      if (MediaQuery.of(context!).viewInsets.bottom != 0) {
        gravity = ToastGravity.CENTER;
      }
    }

    OverlayEntry newEntry = OverlayEntry(builder: (context) {
      if (positionedToastBuilder != null)
        return positionedToastBuilder(context, newChild);
      return _getPostionWidgetBasedOnGravity(newChild, gravity);
    });

    _overlayQueue.add(_ToastEntry(
        entry: newEntry, duration: toastDuration ?? Duration(seconds: 2)));
    if (_timer == null) _showOverlay();
  }

  /// _getPostionWidgetBasedOnGravity generates [Positioned] [Widget]
  /// based on the gravity  [ToastGravity] provided by the user in
  /// [showToast]
  _getPostionWidgetBasedOnGravity(Widget child, ToastGravity? gravity) {
    switch (gravity) {
      case ToastGravity.TOP:
        return Positioned(top: 100.0, left: 24.0, right: 24.0, child: child);
      case ToastGravity.TOP_LEFT:
        return Positioned(top: 100.0, left: 24.0, child: child);
      case ToastGravity.TOP_RIGHT:
        return Positioned(top: 100.0, right: 24.0, child: child);
      case ToastGravity.CENTER:
        return Positioned(
            top: 50.0, bottom: 50.0, left: 24.0, right: 24.0, child: child);
      case ToastGravity.CENTER_LEFT:
        return Positioned(top: 50.0, bottom: 50.0, left: 24.0, child: child);
      case ToastGravity.CENTER_RIGHT:
        return Positioned(top: 50.0, bottom: 50.0, right: 24.0, child: child);
      case ToastGravity.BOTTOM_LEFT:
        return Positioned(bottom: 50.0, left: 24.0, child: child);
      case ToastGravity.BOTTOM_RIGHT:
        return Positioned(bottom: 50.0, right: 24.0, child: child);
      case ToastGravity.SNACKBAR:
        return Positioned(
            bottom: MediaQuery.of(context!).viewInsets.bottom,
            left: 0,
            right: 0,
            child: child);
      case ToastGravity.BOTTOM:
      default:
        return Positioned(bottom: 50.0, left: 24.0, right: 24.0, child: child);
    }
  }
}

/// internal class [_ToastEntry] which maintains
/// each [OverlayEntry] and [Duration] for every toast user
/// triggered
class _ToastEntry {
  final OverlayEntry? entry;
  final Duration? duration;

  _ToastEntry({this.entry, this.duration});
}

/// internal [StatefulWidget] which handles the show and hide
/// animations for [FToast]
class _ToastStateFul extends StatefulWidget {
  _ToastStateFul(this.child, this.duration, {Key? key, this.fadeDuration = 350})
      : super(key: key);

  final Widget child;
  final Duration duration;
  final int fadeDuration;

  @override
  ToastStateFulState createState() => ToastStateFulState();
}

/// State for [_ToastStateFul]
class ToastStateFulState extends State<_ToastStateFul>
    with SingleTickerProviderStateMixin {
  /// Start the showing animations for the toast
  showIt() {
    _animationController!.forward();
  }

  /// Start the hidding animations for the toast
  hideIt() {
    _animationController!.reverse();
    _timer?.cancel();
  }

  /// Controller to start and hide the animation
  AnimationController? _animationController;
  late Animation _fadeAnimation;

  Timer? _timer;

  @override
  void initState() {
    _animationController = AnimationController(
      vsync: this,
      duration: Duration(milliseconds: widget.fadeDuration),
    );
    _fadeAnimation =
        CurvedAnimation(parent: _animationController!, curve: Curves.easeIn);
    super.initState();

    showIt();
    _timer = Timer(widget.duration, () {
      hideIt();
    });
  }

  @override
  void deactivate() {
    _timer?.cancel();
    _animationController!.stop();
    super.deactivate();
  }

  @override
  void dispose() {
    _timer?.cancel();
    _animationController?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: _fadeAnimation as Animation<double>,
      child: Center(
        child: Material(
          color: Colors.transparent,
          child: widget.child,
        ),
      ),
    );
  }
}
```

```dart
import 'dart:async';
import 'dart:html' as html;
import 'package:flutter/services.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';

/// Plugin Class to show a toast message on screen for web
class FluttertoastWebPlugin {
  /// Constructor class
  /// which calls the metohd to inject JS and CSS in to dom
  FluttertoastWebPlugin() {
    injectCssAndJSLibraries();
  }

  /// Registers [MethodChannel] used to communicate with the platform side.
  static void registerWith(Registrar registrar) {
    final MethodChannel channel = MethodChannel(
        'PonnamKarthik/fluttertoast', const StandardMethodCodec(), registrar);
    final FluttertoastWebPlugin instance = FluttertoastWebPlugin();
    channel.setMethodCallHandler(instance.handleMethodCall);
  }

  /// Handle Method Callbacks from [MethodChannel].
  Future<dynamic> handleMethodCall(MethodCall call) async {
    switch (call.method) {
      case 'showToast':
        showToast(call.arguments);
        return true;
      default:
        throw PlatformException(
            code: 'Unimplemented',
            details: "The fluttertoast plugin for web doesn't implement "
                "the method '${call.method}'");
    }
  }

  /// showToast which parses the required arguments and pass
  /// it to [addHtmlToast]
  showToast(args) {
    String msg = args['msg'];
    String? gravity = "top";
    if (args['gravity'] == "top" || args['gravity'] == "bottom") {
      gravity = args["gravity"];
    }

    String position = args['webPosition'] ?? 'right';

    String bgColor =
        args['webBgColor'] ?? "linear-gradient(to right, #00b09b, #96c93d)";

    int? textColor = args['textcolor'];

    int time = args['time'] == null
        ? 3000
        : (int.parse(args['time'].toString()) * 1000);

    bool showClose = args['webShowClose'] ?? false;

    addHtmlToast(
        msg: msg,
        gravity: gravity,
        position: position,
        bgcolor: bgColor,
        showClose: showClose,
        time: time,
        textColor: textColor);
  }

  /// [injectCssAndJSLibraries] which add the JS and CSS files into DOM
  Future<void> injectCssAndJSLibraries() async {
    final List<Future<void>> loading = <Future<void>>[];
    final List<html.HtmlElement> tags = <html.HtmlElement>[];

    final html.LinkElement css = html.LinkElement()
      ..id = 'toast-css'
      ..attributes = {"rel": "stylesheet"}
      ..href = 'assets/packages/fluttertoast/assets/toastify.css';
    tags.add(css);

    final html.ScriptElement script = html.ScriptElement()
      ..async = true
      // ..defer = true
      ..src = "assets/packages/fluttertoast/assets/toastify.js";
    loading.add(script.onLoad.first);
    tags.add(script);
    html.querySelector('head')!.children.addAll(tags);

    await Future.wait(loading);
  }

  /// injects Final [Toastify] code with all the parameters to
  /// make toast visible on web
  addHtmlToast(
      {String msg = "",
      String? gravity = "top",
      String position = "right",
      String bgcolor = "linear-gradient(to right, #00b09b, #96c93d)",
      int time = 3000,
      bool showClose = false,
      int? textColor}) {
    String m = msg.replaceAll("'", "\\'").replaceAll("\n", "<br />");
    html.Element? ele = html.querySelector("#toast-content");
    String content = """
          var toastElement = Toastify({
            text: '$m',
            gravity: '$gravity',
            position: '$position',
            duration: $time,
            close: $showClose,
            backgroundColor: "$bgcolor",
          });
          toastElement.showToast();
        """;
    if (html.querySelector("#toast-content") != null) {
      ele!.remove();
    }
    final html.ScriptElement scriptText = html.ScriptElement()
      ..id = "toast-content"
      ..innerHtml = content;
    html.querySelector('head')!.children.add(scriptText);
    if (textColor != null) {
      html.Element toast = html.querySelector('.toastify')!;
      String tcRadix = textColor.toRadixString(16);
      final String tC = "${tcRadix.substring(2)}${tcRadix.substring(0, 2)}";
      toast.style.setProperty('color', "#$tC");
    }
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_ios/store_kit_wrappers.dart';

final dummyPayment = SKPaymentWrapper(
    productIdentifier: 'prod-id',
    applicationUsername: 'app-user-name',
    requestData: 'fake-data-utf8',
    quantity: 2,
    simulatesAskToBuyInSandbox: true);
final SKError dummyError =
    SKError(code: 111, domain: 'dummy-domain', userInfo: {'key': 'value'});

final SKPaymentTransactionWrapper dummyOriginalTransaction =
    SKPaymentTransactionWrapper(
  transactionState: SKPaymentTransactionStateWrapper.purchased,
  payment: dummyPayment,
  originalTransaction: null,
  transactionTimeStamp: 1231231231.00,
  transactionIdentifier: '123123',
  error: dummyError,
);

final SKPaymentTransactionWrapper dummyTransaction =
    SKPaymentTransactionWrapper(
  transactionState: SKPaymentTransactionStateWrapper.purchased,
  payment: dummyPayment,
  originalTransaction: dummyOriginalTransaction,
  transactionTimeStamp: 1231231231.00,
  transactionIdentifier: '123123',
  error: dummyError,
);

final SKPriceLocaleWrapper dollarLocale = SKPriceLocaleWrapper(
  currencySymbol: '\$',
  currencyCode: 'USD',
  countryCode: 'US',
);

final SKPriceLocaleWrapper noSymbolLocale = SKPriceLocaleWrapper(
  currencySymbol: '',
  currencyCode: 'EUR',
  countryCode: 'UK',
);

final SKProductSubscriptionPeriodWrapper dummySubscription =
    SKProductSubscriptionPeriodWrapper(
  numberOfUnits: 1,
  unit: SKSubscriptionPeriodUnit.month,
);

final SKProductDiscountWrapper dummyDiscount = SKProductDiscountWrapper(
  price: '1.0',
  priceLocale: dollarLocale,
  numberOfPeriods: 1,
  paymentMode: SKProductDiscountPaymentMode.payUpFront,
  subscriptionPeriod: dummySubscription,
);

final SKProductWrapper dummyProductWrapper = SKProductWrapper(
  productIdentifier: 'id',
  localizedTitle: 'title',
  localizedDescription: 'description',
  priceLocale: dollarLocale,
  subscriptionGroupIdentifier: 'com.group',
  price: '1.0',
  subscriptionPeriod: dummySubscription,
  introductoryPrice: dummyDiscount,
);

final SkProductResponseWrapper dummyProductResponseWrapper =
    SkProductResponseWrapper(
  products: [dummyProductWrapper],
  invalidProductIdentifiers: <String>['123'],
);

Map<String, dynamic> buildLocaleMap(SKPriceLocaleWrapper local) {
  return {
    'currencySymbol': local.currencySymbol,
    'currencyCode': local.currencyCode,
    'countryCode': local.countryCode,
  };
}

Map<String, dynamic>? buildSubscriptionPeriodMap(
    SKProductSubscriptionPeriodWrapper? sub) {
  if (sub == null) {
    return null;
  }
  return {
    'numberOfUnits': sub.numberOfUnits,
    'unit': SKSubscriptionPeriodUnit.values.indexOf(sub.unit),
  };
}

Map<String, dynamic> buildDiscountMap(SKProductDiscountWrapper discount) {
  return {
    'price': discount.price,
    'priceLocale': buildLocaleMap(discount.priceLocale),
    'numberOfPeriods': discount.numberOfPeriods,
    'paymentMode':
        SKProductDiscountPaymentMode.values.indexOf(discount.paymentMode),
    'subscriptionPeriod':
        buildSubscriptionPeriodMap(discount.subscriptionPeriod),
  };
}

Map<String, dynamic> buildProductMap(SKProductWrapper product) {
  return {
    'productIdentifier': product.productIdentifier,
    'localizedTitle': product.localizedTitle,
    'localizedDescription': product.localizedDescription,
    'priceLocale': buildLocaleMap(product.priceLocale),
    'subscriptionGroupIdentifier': product.subscriptionGroupIdentifier,
    'price': product.price,
    'subscriptionPeriod':
        buildSubscriptionPeriodMap(product.subscriptionPeriod),
    'introductoryPrice': buildDiscountMap(product.introductoryPrice!),
  };
}

Map<String, dynamic> buildProductResponseMap(
    SkProductResponseWrapper response) {
  List productsMap = response.products
      .map((SKProductWrapper product) => buildProductMap(product))
      .toList();
  return {
    'products': productsMap,
    'invalidProductIdentifiers': response.invalidProductIdentifiers
  };
}

Map<String, dynamic> buildErrorMap(SKError error) {
  return {
    'code': error.code,
    'domain': error.domain,
    'userInfo': error.userInfo,
  };
}

Map<String, dynamic> buildTransactionMap(
    SKPaymentTransactionWrapper transaction) {
  Map<String, dynamic> map = <String, dynamic>{
    'transactionState': SKPaymentTransactionStateWrapper.values
        .indexOf(SKPaymentTransactionStateWrapper.purchased),
    'payment': transaction.payment.toMap(),
    'originalTransaction': transaction.originalTransaction == null
        ? null
        : buildTransactionMap(transaction.originalTransaction!),
    'transactionTimeStamp': transaction.transactionTimeStamp,
    'transactionIdentifier': transaction.transactionIdentifier,
    'error': buildErrorMap(transaction.error!),
  };
  return map;
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_ios/src/types/app_store_product_details.dart';
import 'package:in_app_purchase_ios/src/types/app_store_purchase_details.dart';
import 'package:in_app_purchase_ios/src/store_kit_wrappers/sk_product_wrapper.dart';
import 'package:in_app_purchase_ios/store_kit_wrappers.dart';
import 'package:test/test.dart';

import 'sk_test_stub_objects.dart';

void main() {
  group('product related object wrapper test', () {
    test(
        'SKProductSubscriptionPeriodWrapper should have property values consistent with map',
        () {
      final SKProductSubscriptionPeriodWrapper wrapper =
          SKProductSubscriptionPeriodWrapper.fromJson(
              buildSubscriptionPeriodMap(dummySubscription)!);
      expect(wrapper, equals(dummySubscription));
    });

    test(
        'SKProductSubscriptionPeriodWrapper should have properties to be default values if map is empty',
        () {
      final SKProductSubscriptionPeriodWrapper wrapper =
          SKProductSubscriptionPeriodWrapper.fromJson(<String, dynamic>{});
      expect(wrapper.numberOfUnits, 0);
      expect(wrapper.unit, SKSubscriptionPeriodUnit.day);
    });

    test(
        'SKProductDiscountWrapper should have property values consistent with map',
        () {
      final SKProductDiscountWrapper wrapper =
          SKProductDiscountWrapper.fromJson(buildDiscountMap(dummyDiscount));
      expect(wrapper, equals(dummyDiscount));
    });

    test(
        'SKProductDiscountWrapper should have properties to be default if map is empty',
        () {
      final SKProductDiscountWrapper wrapper =
          SKProductDiscountWrapper.fromJson(<String, dynamic>{});
      expect(wrapper.price, '');
      expect(
          wrapper.priceLocale,
          SKPriceLocaleWrapper(
            currencyCode: '',
            currencySymbol: '',
            countryCode: '',
          ));
      expect(wrapper.numberOfPeriods, 0);
      expect(wrapper.paymentMode, SKProductDiscountPaymentMode.payAsYouGo);
      expect(
          wrapper.subscriptionPeriod,
          SKProductSubscriptionPeriodWrapper(
              numberOfUnits: 0, unit: SKSubscriptionPeriodUnit.day));
    });

    test('SKProductWrapper should have property values consistent with map',
        () {
      final SKProductWrapper wrapper =
          SKProductWrapper.fromJson(buildProductMap(dummyProductWrapper));
      expect(wrapper, equals(dummyProductWrapper));
    });

    test(
        'SKProductWrapper should have properties to be default if map is empty',
        () {
      final SKProductWrapper wrapper =
          SKProductWrapper.fromJson(<String, dynamic>{});
      expect(wrapper.productIdentifier, '');
      expect(wrapper.localizedTitle, '');
      expect(wrapper.localizedDescription, '');
      expect(
          wrapper.priceLocale,
          SKPriceLocaleWrapper(
            currencyCode: '',
            currencySymbol: '',
            countryCode: '',
          ));
      expect(wrapper.subscriptionGroupIdentifier, null);
      expect(wrapper.price, '');
      expect(wrapper.subscriptionPeriod, null);
    });

    test('toProductDetails() should return correct Product object', () {
      final SKProductWrapper wrapper =
          SKProductWrapper.fromJson(buildProductMap(dummyProductWrapper));
      final AppStoreProductDetails product =
          AppStoreProductDetails.fromSKProduct(wrapper);
      expect(product.title, wrapper.localizedTitle);
      expect(product.description, wrapper.localizedDescription);
      expect(product.id, wrapper.productIdentifier);
      expect(product.price,
          wrapper.priceLocale.currencySymbol + wrapper.price.toString());
      expect(product.skProduct, wrapper);
    });

    test('SKProductResponse wrapper should match', () {
      final SkProductResponseWrapper wrapper =
          SkProductResponseWrapper.fromJson(
              buildProductResponseMap(dummyProductResponseWrapper));
      expect(wrapper, equals(dummyProductResponseWrapper));
    });
    test('SKProductResponse wrapper should default to empty list', () {
      final Map<String, List<dynamic>> productResponseMapEmptyList =
          <String, List<dynamic>>{
        'products': <Map<String, dynamic>>[],
        'invalidProductIdentifiers': <String>[],
      };
      final SkProductResponseWrapper wrapper =
          SkProductResponseWrapper.fromJson(productResponseMapEmptyList);
      expect(wrapper.products.length, 0);
      expect(wrapper.invalidProductIdentifiers.length, 0);
    });

    test('LocaleWrapper should have property values consistent with map', () {
      final SKPriceLocaleWrapper wrapper =
          SKPriceLocaleWrapper.fromJson(buildLocaleMap(dollarLocale));
      expect(wrapper, equals(dollarLocale));
    });
  });

  group('Payment queue related object tests', () {
    test('Should construct correct SKPaymentWrapper from json', () {
      SKPaymentWrapper payment =
          SKPaymentWrapper.fromJson(dummyPayment.toMap());
      expect(payment, equals(dummyPayment));
    });

    test('Should construct correct SKError from json', () {
      SKError error = SKError.fromJson(buildErrorMap(dummyError));
      expect(error, equals(dummyError));
    });

    test('Should construct correct SKTransactionWrapper from json', () {
      SKPaymentTransactionWrapper transaction =
          SKPaymentTransactionWrapper.fromJson(
              buildTransactionMap(dummyTransaction));
      expect(transaction, equals(dummyTransaction));
    });

    test('toPurchaseDetails() should return correct PurchaseDetail object', () {
      AppStorePurchaseDetails details =
          AppStorePurchaseDetails.fromSKTransaction(
              dummyTransaction, 'receipt data');
      expect(dummyTransaction.transactionIdentifier, details.purchaseID);
      expect(dummyTransaction.payment.productIdentifier, details.productID);
      expect(dummyTransaction.transactionTimeStamp, isNotNull);
      expect((dummyTransaction.transactionTimeStamp! * 1000).toInt().toString(),
          details.transactionDate);
      expect(details.verificationData.localVerificationData, 'receipt data');
      expect(details.verificationData.serverVerificationData, 'receipt data');
      expect(details.verificationData.source, 'app_store');
      expect(details.skPaymentTransaction, dummyTransaction);
      expect(details.pendingCompletePurchase, true);
    });

    test('SKPaymentTransactionWrapper.toFinishMap set correct value', () {
      final SKPaymentTransactionWrapper transactionWrapper =
          SKPaymentTransactionWrapper(
              payment: dummyPayment,
              transactionState: SKPaymentTransactionStateWrapper.failed,
              transactionIdentifier: 'abcd');
      final Map<String, String?> finishMap = transactionWrapper.toFinishMap();
      expect(finishMap['transactionIdentifier'], 'abcd');
      expect(finishMap['productIdentifier'], dummyPayment.productIdentifier);
    });

    test(
        'SKPaymentTransactionWrapper.toFinishMap should set transactionIdentifier to null when necessary',
        () {
      final SKPaymentTransactionWrapper transactionWrapper =
          SKPaymentTransactionWrapper(
              payment: dummyPayment,
              transactionState: SKPaymentTransactionStateWrapper.failed);
      final Map<String, String?> finishMap = transactionWrapper.toFinishMap();
      expect(finishMap['transactionIdentifier'], null);
    });

    test('Should generate correct map of the payment object', () {
      Map map = dummyPayment.toMap();
      expect(map['productIdentifier'], dummyPayment.productIdentifier);
      expect(map['applicationUsername'], dummyPayment.applicationUsername);

      expect(map['requestData'], dummyPayment.requestData);

      expect(map['quantity'], dummyPayment.quantity);

      expect(map['simulatesAskToBuyInSandbox'],
          dummyPayment.simulatesAskToBuyInSandbox);
    });
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:in_app_purchase_ios/src/channel.dart';
import 'package:in_app_purchase_ios/store_kit_wrappers.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final FakeIOSPlatform fakeIOSPlatform = FakeIOSPlatform();

  setUpAll(() {
    SystemChannels.platform
        .setMockMethodCallHandler(fakeIOSPlatform.onMethodCall);
  });

  test(
      'handlePaymentQueueDelegateCallbacks should call SKPaymentQueueDelegateWrapper.shouldContinueTransaction',
      () async {
    SKPaymentQueueWrapper queue = SKPaymentQueueWrapper();
    TestPaymentQueueDelegate testDelegate = TestPaymentQueueDelegate();
    await queue.setDelegate(testDelegate);

    final Map<String, dynamic> arguments = <String, dynamic>{
      'storefront': <String, String>{
        'countryCode': 'USA',
        'identifier': 'unique_identifier',
      },
      'transaction': <String, dynamic>{
        'payment': <String, dynamic>{
          'productIdentifier': 'product_identifier',
        }
      },
    };

    final result = await queue.handlePaymentQueueDelegateCallbacks(
      MethodCall('shouldContinueTransaction', arguments),
    );

    expect(result, false);
    expect(
      testDelegate.log,
      <Matcher>{
        equals('shouldContinueTransaction'),
      },
    );
  });

  test(
      'handlePaymentQueueDelegateCallbacks should call SKPaymentQueueDelegateWrapper.shouldShowPriceConsent',
      () async {
    SKPaymentQueueWrapper queue = SKPaymentQueueWrapper();
    TestPaymentQueueDelegate testDelegate = TestPaymentQueueDelegate();
    await queue.setDelegate(testDelegate);

    final result = await queue.handlePaymentQueueDelegateCallbacks(
      MethodCall('shouldShowPriceConsent'),
    );

    expect(result, false);
    expect(
      testDelegate.log,
      <Matcher>{
        equals('shouldShowPriceConsent'),
      },
    );
  });

  test(
      'handleObserverCallbacks should call SKTransactionObserverWrapper.restoreCompletedTransactionsFailed',
      () async {
    SKPaymentQueueWrapper queue = SKPaymentQueueWrapper();
    TestTransactionObserverWrapper testObserver =
        TestTransactionObserverWrapper();
    queue.setTransactionObserver(testObserver);

    final arguments = <dynamic, dynamic>{
      'code': 100,
      'domain': 'domain',
      'userInfo': <String, dynamic>{'error': 'underlying_error'},
    };

    await queue.handleObserverCallbacks(
      MethodCall('restoreCompletedTransactionsFailed', arguments),
    );

    expect(
      testObserver.log,
      <Matcher>{
        equals('restoreCompletedTransactionsFailed'),
      },
    );
  });
}

class TestTransactionObserverWrapper extends SKTransactionObserverWrapper {
  final List<String> log = <String>[];

  @override
  void updatedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions}) {
    log.add('updatedTransactions');
  }

  @override
  void removedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions}) {
    log.add('removedTransactions');
  }

  @override
  void restoreCompletedTransactionsFailed({required SKError error}) {
    log.add('restoreCompletedTransactionsFailed');
  }

  @override
  void paymentQueueRestoreCompletedTransactionsFinished() {
    log.add('paymentQueueRestoreCompletedTransactionsFinished');
  }

  @override
  bool shouldAddStorePayment(
      {required SKPaymentWrapper payment, required SKProductWrapper product}) {
    log.add('shouldAddStorePayment');
    return false;
  }
}

class TestPaymentQueueDelegate extends SKPaymentQueueDelegateWrapper {
  final List<String> log = <String>[];

  @override
  bool shouldContinueTransaction(
      SKPaymentTransactionWrapper transaction, SKStorefrontWrapper storefront) {
    log.add('shouldContinueTransaction');
    return false;
  }

  @override
  bool shouldShowPriceConsent() {
    log.add('shouldShowPriceConsent');
    return false;
  }
}

class FakeIOSPlatform {
  FakeIOSPlatform() {
    channel.setMockMethodCallHandler(onMethodCall);
  }

  // indicate if the payment queue delegate is registered
  bool isPaymentQueueDelegateRegistered = false;

  Future<dynamic> onMethodCall(MethodCall call) {
    switch (call.method) {
      case '-[SKPaymentQueue registerDelegate]':
        isPaymentQueueDelegateRegistered = true;
        return Future<void>.sync(() {});
      case '-[SKPaymentQueue removeDelegate]':
        isPaymentQueueDelegateRegistered = false;
        return Future<void>.sync(() {});
    }
    return Future.error('method not mocked');
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:in_app_purchase_ios/src/channel.dart';
import 'package:in_app_purchase_ios/store_kit_wrappers.dart';
import 'sk_test_stub_objects.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final FakeIOSPlatform fakeIOSPlatform = FakeIOSPlatform();

  setUpAll(() {
    SystemChannels.platform
        .setMockMethodCallHandler(fakeIOSPlatform.onMethodCall);
  });

  setUp(() {});

  tearDown(() {
    fakeIOSPlatform.testReturnNull = false;
    fakeIOSPlatform.queueIsActive = null;
    fakeIOSPlatform.getReceiptFailTest = false;
  });

  group('sk_request_maker', () {
    test('get products method channel', () async {
      SkProductResponseWrapper productResponseWrapper =
          await SKRequestMaker().startProductRequest(['xxx']);
      expect(
        productResponseWrapper.products,
        isNotEmpty,
      );
      expect(
        productResponseWrapper.products.first.priceLocale.currencySymbol,
        '\$',
      );

      expect(
        productResponseWrapper.products.first.priceLocale.currencySymbol,
        isNot('A'),
      );
      expect(
        productResponseWrapper.products.first.priceLocale.currencyCode,
        'USD',
      );
      expect(
        productResponseWrapper.products.first.priceLocale.countryCode,
        'US',
      );
      expect(
        productResponseWrapper.invalidProductIdentifiers,
        isNotEmpty,
      );

      expect(
        fakeIOSPlatform.startProductRequestParam,
        ['xxx'],
      );
    });

    test('get products method channel should throw exception', () async {
      fakeIOSPlatform.getProductRequestFailTest = true;
      expect(
        SKRequestMaker().startProductRequest(<String>['xxx']),
        throwsException,
      );
      fakeIOSPlatform.getProductRequestFailTest = false;
    });

    test('refreshed receipt', () async {
      int receiptCountBefore = fakeIOSPlatform.refreshReceipt;
      await SKRequestMaker().startRefreshReceiptRequest(
          receiptProperties: <String, dynamic>{"isExpired": true});
      expect(fakeIOSPlatform.refreshReceipt, receiptCountBefore + 1);
      expect(fakeIOSPlatform.refreshReceiptParam,
          <String, dynamic>{"isExpired": true});
    });

    test('should get null receipt if any exceptions are raised', () async {
      fakeIOSPlatform.getReceiptFailTest = true;
      expect(() async => SKReceiptManager.retrieveReceiptData(),
          throwsA(TypeMatcher<PlatformException>()));
    });
  });

  group('sk_receipt_manager', () {
    test('should get receipt (faking it by returning a `receipt data` string)',
        () async {
      String receiptData = await SKReceiptManager.retrieveReceiptData();
      expect(receiptData, 'receipt data');
    });
  });

  group('sk_payment_queue', () {
    test('canMakePayment should return true', () async {
      expect(await SKPaymentQueueWrapper.canMakePayments(), true);
    });

    test('canMakePayment returns false if method channel returns null',
        () async {
      fakeIOSPlatform.testReturnNull = true;
      expect(await SKPaymentQueueWrapper.canMakePayments(), false);
    });

    test('transactions should return a valid list of transactions', () async {
      expect(await SKPaymentQueueWrapper().transactions(), isNotEmpty);
    });

    test(
        'throws if observer is not set for payment queue before adding payment',
        () async {
      expect(SKPaymentQueueWrapper().addPayment(dummyPayment),
          throwsAssertionError);
    });

    test('should add payment to the payment queue', () async {
      SKPaymentQueueWrapper queue = SKPaymentQueueWrapper();
      TestPaymentTransactionObserver observer =
          TestPaymentTransactionObserver();
      queue.setTransactionObserver(observer);
      await queue.addPayment(dummyPayment);
      expect(fakeIOSPlatform.payments.first, equals(dummyPayment));
    });

    test('should finish transaction', () async {
      SKPaymentQueueWrapper queue = SKPaymentQueueWrapper();
      TestPaymentTransactionObserver observer =
          TestPaymentTransactionObserver();
      queue.setTransactionObserver(observer);
      await queue.finishTransaction(dummyTransaction);
      expect(fakeIOSPlatform.transactionsFinished.first,
          equals(dummyTransaction.toFinishMap()));
    });

    test('should restore transaction', () async {
      SKPaymentQueueWrapper queue = SKPaymentQueueWrapper();
      TestPaymentTransactionObserver observer =
          TestPaymentTransactionObserver();
      queue.setTransactionObserver(observer);
      await queue.restoreTransactions(applicationUserName: 'aUserID');
      expect(fakeIOSPlatform.applicationNameHasTransactionRestored, 'aUserID');
    });

    test('startObservingTransactionQueue should call methodChannel', () async {
      expect(fakeIOSPlatform.queueIsActive, isNot(true));
      await SKPaymentQueueWrapper().startObservingTransactionQueue();
      expect(fakeIOSPlatform.queueIsActive, true);
    });

    test('stopObservingTransactionQueue should call methodChannel', () async {
      expect(fakeIOSPlatform.queueIsActive, isNot(false));
      await SKPaymentQueueWrapper().stopObservingTransactionQueue();
      expect(fakeIOSPlatform.queueIsActive, false);
    });

    test('setDelegate should call methodChannel', () async {
      expect(fakeIOSPlatform.isPaymentQueueDelegateRegistered, false);
      await SKPaymentQueueWrapper().setDelegate(TestPaymentQueueDelegate());
      expect(fakeIOSPlatform.isPaymentQueueDelegateRegistered, true);
      await SKPaymentQueueWrapper().setDelegate(null);
      expect(fakeIOSPlatform.isPaymentQueueDelegateRegistered, false);
    });

    test('showPriceConsentIfNeeded should call methodChannel', () async {
      expect(fakeIOSPlatform.showPriceConsentIfNeeded, false);
      await SKPaymentQueueWrapper().showPriceConsentIfNeeded();
      expect(fakeIOSPlatform.showPriceConsentIfNeeded, true);
    });
  });

  group('Code Redemption Sheet', () {
    test('presentCodeRedemptionSheet should not throw', () async {
      expect(fakeIOSPlatform.presentCodeRedemption, false);
      await SKPaymentQueueWrapper().presentCodeRedemptionSheet();
      expect(fakeIOSPlatform.presentCodeRedemption, true);
      fakeIOSPlatform.presentCodeRedemption = false;
    });
  });
}

class FakeIOSPlatform {
  FakeIOSPlatform() {
    channel.setMockMethodCallHandler(onMethodCall);
  }
  // get product request
  List<dynamic> startProductRequestParam = [];
  bool getProductRequestFailTest = false;
  bool testReturnNull = false;

  // get receipt request
  bool getReceiptFailTest = false;

  // refresh receipt request
  int refreshReceipt = 0;
  late Map<String, dynamic> refreshReceiptParam;

  // payment queue
  List<SKPaymentWrapper> payments = [];
  List<Map<String, String>> transactionsFinished = [];
  String applicationNameHasTransactionRestored = '';

  // present Code Redemption
  bool presentCodeRedemption = false;

  // show price consent sheet
  bool showPriceConsentIfNeeded = false;

  // indicate if the payment queue delegate is registered
  bool isPaymentQueueDelegateRegistered = false;

  // Listen to purchase updates
  bool? queueIsActive;

  Future<dynamic> onMethodCall(MethodCall call) {
    switch (call.method) {
      // request makers
      case '-[InAppPurchasePlugin startProductRequest:result:]':
        startProductRequestParam = call.arguments;
        if (getProductRequestFailTest) {
          return Future<dynamic>.value(null);
        }
        return Future<Map<String, dynamic>>.value(
            buildProductResponseMap(dummyProductResponseWrapper));
      case '-[InAppPurchasePlugin refreshReceipt:result:]':
        refreshReceipt++;
        refreshReceiptParam =
            Map.castFrom<dynamic, dynamic, String, dynamic>(call.arguments);
        return Future<void>.sync(() {});
      // receipt manager
      case '-[InAppPurchasePlugin retrieveReceiptData:result:]':
        if (getReceiptFailTest) {
          throw ("some arbitrary error");
        }
        return Future<String>.value('receipt data');
      // payment queue
      case '-[SKPaymentQueue canMakePayments:]':
        if (testReturnNull) {
          return Future<dynamic>.value(null);
        }
        return Future<bool>.value(true);
      case '-[SKPaymentQueue transactions]':
        return Future<List<dynamic>>.value(
            [buildTransactionMap(dummyTransaction)]);
      case '-[InAppPurchasePlugin addPayment:result:]':
        payments.add(SKPaymentWrapper.fromJson(
            Map<String, dynamic>.from(call.arguments)));
        return Future<void>.sync(() {});
      case '-[InAppPurchasePlugin finishTransaction:result:]':
        transactionsFinished.add(Map<String, String>.from(call.arguments));
        return Future<void>.sync(() {});
      case '-[InAppPurchasePlugin restoreTransactions:result:]':
        applicationNameHasTransactionRestored = call.arguments;
        return Future<void>.sync(() {});
      case '-[InAppPurchasePlugin presentCodeRedemptionSheet:result:]':
        presentCodeRedemption = true;
        return Future<void>.sync(() {});
      case '-[SKPaymentQueue startObservingTransactionQueue]':
        queueIsActive = true;
        return Future<void>.sync(() {});
      case '-[SKPaymentQueue stopObservingTransactionQueue]':
        queueIsActive = false;
        return Future<void>.sync(() {});
      case '-[SKPaymentQueue registerDelegate]':
        isPaymentQueueDelegateRegistered = true;
        return Future<void>.sync(() {});
      case '-[SKPaymentQueue removeDelegate]':
        isPaymentQueueDelegateRegistered = false;
        return Future<void>.sync(() {});
      case '-[SKPaymentQueue showPriceConsentIfNeeded]':
        showPriceConsentIfNeeded = true;
        return Future<void>.sync(() {});
    }
    return Future.error('method not mocked');
  }
}

class TestPaymentQueueDelegate extends SKPaymentQueueDelegateWrapper {}

class TestPaymentTransactionObserver extends SKTransactionObserverWrapper {
  void updatedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions}) {}

  void removedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions}) {}

  void restoreCompletedTransactionsFailed({required SKError error}) {}

  void paymentQueueRestoreCompletedTransactionsFinished() {}

  bool shouldAddStorePayment(
      {required SKPaymentWrapper payment, required SKProductWrapper product}) {
    return true;
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:in_app_purchase_ios/in_app_purchase_ios.dart';
import 'package:in_app_purchase_ios/src/store_kit_wrappers/enum_converters.dart';
import 'package:in_app_purchase_ios/store_kit_wrappers.dart';
import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import 'fakes/fake_ios_platform.dart';
import 'store_kit_wrappers/sk_test_stub_objects.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final FakeIOSPlatform fakeIOSPlatform = FakeIOSPlatform();
  late InAppPurchaseIosPlatform iapIosPlatform;

  setUpAll(() {
    SystemChannels.platform
        .setMockMethodCallHandler(fakeIOSPlatform.onMethodCall);
  });

  setUp(() {
    InAppPurchaseIosPlatform.registerPlatform();
    iapIosPlatform = InAppPurchasePlatform.instance as InAppPurchaseIosPlatform;
    fakeIOSPlatform.reset();
  });

  tearDown(() => fakeIOSPlatform.reset());

  group('isAvailable', () {
    test('true', () async {
      expect(await iapIosPlatform.isAvailable(), isTrue);
    });
  });

  group('query product list', () {
    test('should get product list and correct invalid identifiers', () async {
      final InAppPurchaseIosPlatform connection = InAppPurchaseIosPlatform();
      final ProductDetailsResponse response = await connection
          .queryProductDetails(<String>['123', '456', '789'].toSet());
      List<ProductDetails> products = response.productDetails;
      expect(products.first.id, '123');
      expect(products[1].id, '456');
      expect(response.notFoundIDs, ['789']);
      expect(response.error, isNull);
      expect(response.productDetails.first.currencySymbol, r'$');
      expect(response.productDetails[1].currencySymbol, 'EUR');
    });

    test(
        'if query products throws error, should get error object in the response',
        () async {
      fakeIOSPlatform.queryProductException = PlatformException(
          code: 'error_code',
          message: 'error_message',
          details: {'info': 'error_info'});
      final InAppPurchaseIosPlatform connection = InAppPurchaseIosPlatform();
      final ProductDetailsResponse response = await connection
          .queryProductDetails(<String>['123', '456', '789'].toSet());
      expect(response.productDetails, []);
      expect(response.notFoundIDs, ['123', '456', '789']);
      expect(response.error, isNotNull);
      expect(response.error!.source, kIAPSource);
      expect(response.error!.code, 'error_code');
      expect(response.error!.message, 'error_message');
      expect(response.error!.details, {'info': 'error_info'});
    });
  });

  group('restore purchases', () {
    test('should emit restored transactions on purchase stream', () async {
      Completer completer = Completer();
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;

      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        if (purchaseDetailsList.first.status == PurchaseStatus.restored) {
          completer.complete(purchaseDetailsList);
          subscription.cancel();
        }
      });

      await iapIosPlatform.restorePurchases();
      List<PurchaseDetails> details = await completer.future;

      expect(details.length, 2);
      for (int i = 0; i < fakeIOSPlatform.transactions.length; i++) {
        SKPaymentTransactionWrapper expected = fakeIOSPlatform.transactions[i];
        PurchaseDetails actual = details[i];

        expect(actual.purchaseID, expected.transactionIdentifier);
        expect(actual.verificationData, isNotNull);
        expect(actual.status, PurchaseStatus.restored);
        expect(actual.verificationData.localVerificationData,
            fakeIOSPlatform.receiptData);
        expect(actual.verificationData.serverVerificationData,
            fakeIOSPlatform.receiptData);
        expect(actual.pendingCompletePurchase, true);
      }
    });

    test('should not block transaction updates', () async {
      fakeIOSPlatform.transactions
          .insert(0, fakeIOSPlatform.createPurchasedTransaction('foo', 'bar'));
      Completer completer = Completer();
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;

      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        if (purchaseDetailsList.first.status == PurchaseStatus.purchased) {
          completer.complete(purchaseDetailsList);
          subscription.cancel();
        }
      });
      await iapIosPlatform.restorePurchases();
      List<PurchaseDetails> details = await completer.future;
      expect(details.length, 3);
      for (int i = 0; i < fakeIOSPlatform.transactions.length; i++) {
        SKPaymentTransactionWrapper expected = fakeIOSPlatform.transactions[i];
        PurchaseDetails actual = details[i];

        expect(actual.purchaseID, expected.transactionIdentifier);
        expect(actual.verificationData, isNotNull);
        expect(
          actual.status,
          SKTransactionStatusConverter()
              .toPurchaseStatus(expected.transactionState),
        );
        expect(actual.verificationData.localVerificationData,
            fakeIOSPlatform.receiptData);
        expect(actual.verificationData.serverVerificationData,
            fakeIOSPlatform.receiptData);
        expect(actual.pendingCompletePurchase, true);
      }
    });

    test('receipt error should populate null to verificationData.data',
        () async {
      fakeIOSPlatform.receiptData = null;
      Completer completer = Completer();
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;

      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        if (purchaseDetailsList.first.status == PurchaseStatus.restored) {
          completer.complete(purchaseDetailsList);
          subscription.cancel();
        }
      });

      await iapIosPlatform.restorePurchases();
      List<PurchaseDetails> details = await completer.future;

      for (PurchaseDetails purchase in details) {
        expect(purchase.verificationData.localVerificationData, isEmpty);
        expect(purchase.verificationData.serverVerificationData, isEmpty);
      }
    });

    test('test restore error', () {
      fakeIOSPlatform.testRestoredError = SKError(
          code: 123,
          domain: 'error_test',
          userInfo: {'message': 'errorMessage'});

      expect(
          () => iapIosPlatform.restorePurchases(),
          throwsA(
            isA<SKError>()
                .having((error) => error.code, 'code', 123)
                .having((error) => error.domain, 'domain', 'error_test')
                .having((error) => error.userInfo, 'userInfo',
                    {'message': 'errorMessage'}),
          ));
    });
  });

  group('make payment', () {
    test(
        'buying non consumable, should get purchase objects in the purchase update callback',
        () async {
      List<PurchaseDetails> details = [];
      Completer completer = Completer();
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;

      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        details.addAll(purchaseDetailsList);
        if (purchaseDetailsList.first.status == PurchaseStatus.purchased) {
          completer.complete(details);
          subscription.cancel();
        }
      });
      final AppStorePurchaseParam purchaseParam = AppStorePurchaseParam(
          productDetails:
              AppStoreProductDetails.fromSKProduct(dummyProductWrapper),
          applicationUserName: 'appName');
      await iapIosPlatform.buyNonConsumable(purchaseParam: purchaseParam);

      List<PurchaseDetails> result = await completer.future;
      expect(result.length, 2);
      expect(result.first.productID, dummyProductWrapper.productIdentifier);
    });

    test(
        'buying consumable, should get purchase objects in the purchase update callback',
        () async {
      List<PurchaseDetails> details = [];
      Completer completer = Completer();
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;

      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        details.addAll(purchaseDetailsList);
        if (purchaseDetailsList.first.status == PurchaseStatus.purchased) {
          completer.complete(details);
          subscription.cancel();
        }
      });
      final AppStorePurchaseParam purchaseParam = AppStorePurchaseParam(
          productDetails:
              AppStoreProductDetails.fromSKProduct(dummyProductWrapper),
          applicationUserName: 'appName');
      await iapIosPlatform.buyConsumable(purchaseParam: purchaseParam);

      List<PurchaseDetails> result = await completer.future;
      expect(result.length, 2);
      expect(result.first.productID, dummyProductWrapper.productIdentifier);
    });

    test('buying consumable, should throw when autoConsume is false', () async {
      final AppStorePurchaseParam purchaseParam = AppStorePurchaseParam(
          productDetails:
              AppStoreProductDetails.fromSKProduct(dummyProductWrapper),
          applicationUserName: 'appName');
      expect(
          () => iapIosPlatform.buyConsumable(
              purchaseParam: purchaseParam, autoConsume: false),
          throwsA(isInstanceOf<AssertionError>()));
    });

    test('should get failed purchase status', () async {
      fakeIOSPlatform.testTransactionFail = true;
      List<PurchaseDetails> details = [];
      Completer completer = Completer();
      late IAPError error;

      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;
      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        details.addAll(purchaseDetailsList);
        purchaseDetailsList.forEach((purchaseDetails) {
          if (purchaseDetails.status == PurchaseStatus.error) {
            error = purchaseDetails.error!;
            completer.complete(error);
            subscription.cancel();
          }
        });
      });
      final AppStorePurchaseParam purchaseParam = AppStorePurchaseParam(
          productDetails:
              AppStoreProductDetails.fromSKProduct(dummyProductWrapper),
          applicationUserName: 'appName');
      await iapIosPlatform.buyNonConsumable(purchaseParam: purchaseParam);

      IAPError completerError = await completer.future;
      expect(completerError.code, 'purchase_error');
      expect(completerError.source, kIAPSource);
      expect(completerError.message, 'ios_domain');
      expect(completerError.details, {'message': 'an error message'});
    });
  });

  group('complete purchase', () {
    test('should complete purchase', () async {
      List<PurchaseDetails> details = [];
      Completer completer = Completer();
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;
      late StreamSubscription subscription;
      subscription = stream.listen((purchaseDetailsList) {
        details.addAll(purchaseDetailsList);
        purchaseDetailsList.forEach((purchaseDetails) {
          if (purchaseDetails.pendingCompletePurchase) {
            iapIosPlatform.completePurchase(purchaseDetails);
            completer.complete(details);
            subscription.cancel();
          }
        });
      });
      final AppStorePurchaseParam purchaseParam = AppStorePurchaseParam(
          productDetails:
              AppStoreProductDetails.fromSKProduct(dummyProductWrapper),
          applicationUserName: 'appName');
      await iapIosPlatform.buyNonConsumable(purchaseParam: purchaseParam);
      List<PurchaseDetails> result = await completer.future;
      expect(result.length, 2);
      expect(result.first.productID, dummyProductWrapper.productIdentifier);
      expect(fakeIOSPlatform.finishedTransactions.length, 1);
    });
  });

  group('purchase stream', () {
    test('Should only have active queue when purchaseStream has listeners', () {
      Stream<List<PurchaseDetails>> stream = iapIosPlatform.purchaseStream;
      expect(fakeIOSPlatform.queueIsActive, false);
      StreamSubscription subscription1 = stream.listen((event) {});
      expect(fakeIOSPlatform.queueIsActive, true);
      StreamSubscription subscription2 = stream.listen((event) {});
      expect(fakeIOSPlatform.queueIsActive, true);
      subscription1.cancel();
      expect(fakeIOSPlatform.queueIsActive, true);
      subscription2.cancel();
      expect(fakeIOSPlatform.queueIsActive, false);
    });
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:io';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:in_app_purchase_ios/in_app_purchase_ios.dart';
import 'package:in_app_purchase_ios/src/channel.dart';
import 'package:in_app_purchase_ios/store_kit_wrappers.dart';

import '../store_kit_wrappers/sk_test_stub_objects.dart';

class FakeIOSPlatform {
  FakeIOSPlatform() {
    channel.setMockMethodCallHandler(onMethodCall);
  }

  // pre-configured store informations
  String? receiptData;
  late Set<String> validProductIDs;
  late Map<String, SKProductWrapper> validProducts;
  late List<SKPaymentTransactionWrapper> transactions;
  late List<SKPaymentTransactionWrapper> finishedTransactions;
  late bool testRestoredTransactionsNull;
  late bool testTransactionFail;
  PlatformException? queryProductException;
  PlatformException? restoreException;
  SKError? testRestoredError;
  bool queueIsActive = false;

  void reset() {
    transactions = [];
    receiptData = 'dummy base64data';
    validProductIDs = ['123', '456'].toSet();
    validProducts = Map();
    for (String validID in validProductIDs) {
      Map<String, dynamic> productWrapperMap =
          buildProductMap(dummyProductWrapper);
      productWrapperMap['productIdentifier'] = validID;
      if (validID == '456') {
        productWrapperMap['priceLocale'] = buildLocaleMap(noSymbolLocale);
      }
      validProducts[validID] = SKProductWrapper.fromJson(productWrapperMap);
    }

    SKPaymentTransactionWrapper tran1 = SKPaymentTransactionWrapper(
      transactionIdentifier: '123',
      payment: dummyPayment,
      originalTransaction: dummyTransaction,
      transactionTimeStamp: 123123123.022,
      transactionState: SKPaymentTransactionStateWrapper.restored,
      error: null,
    );
    SKPaymentTransactionWrapper tran2 = SKPaymentTransactionWrapper(
      transactionIdentifier: '1234',
      payment: dummyPayment,
      originalTransaction: dummyTransaction,
      transactionTimeStamp: 123123123.022,
      transactionState: SKPaymentTransactionStateWrapper.restored,
      error: null,
    );

    transactions.addAll([tran1, tran2]);
    finishedTransactions = [];
    testRestoredTransactionsNull = false;
    testTransactionFail = false;
    queryProductException = null;
    restoreException = null;
    testRestoredError = null;
    queueIsActive = false;
  }

  SKPaymentTransactionWrapper createPendingTransaction(String id) {
    return SKPaymentTransactionWrapper(
        transactionIdentifier: '',
        payment: SKPaymentWrapper(productIdentifier: id),
        transactionState: SKPaymentTransactionStateWrapper.purchasing,
        transactionTimeStamp: 123123.121,
        error: null,
        originalTransaction: null);
  }

  SKPaymentTransactionWrapper createPurchasedTransaction(
      String productId, String transactionId) {
    return SKPaymentTransactionWrapper(
        payment: SKPaymentWrapper(productIdentifier: productId),
        transactionState: SKPaymentTransactionStateWrapper.purchased,
        transactionTimeStamp: 123123.121,
        transactionIdentifier: transactionId,
        error: null,
        originalTransaction: null);
  }

  SKPaymentTransactionWrapper createFailedTransaction(String productId) {
    return SKPaymentTransactionWrapper(
        transactionIdentifier: '',
        payment: SKPaymentWrapper(productIdentifier: productId),
        transactionState: SKPaymentTransactionStateWrapper.failed,
        transactionTimeStamp: 123123.121,
        error: SKError(
            code: 0,
            domain: 'ios_domain',
            userInfo: {'message': 'an error message'}),
        originalTransaction: null);
  }

  Future<dynamic> onMethodCall(MethodCall call) {
    switch (call.method) {
      case '-[SKPaymentQueue canMakePayments:]':
        return Future<bool>.value(true);
      case '-[InAppPurchasePlugin startProductRequest:result:]':
        if (queryProductException != null) {
          throw queryProductException!;
        }
        List<String> productIDS =
            List.castFrom<dynamic, String>(call.arguments);
        List<String> invalidFound = [];
        List<SKProductWrapper> products = [];
        for (String productID in productIDS) {
          if (!validProductIDs.contains(productID)) {
            invalidFound.add(productID);
          } else {
            products.add(validProducts[productID]!);
          }
        }
        SkProductResponseWrapper response = SkProductResponseWrapper(
            products: products, invalidProductIdentifiers: invalidFound);
        return Future<Map<String, dynamic>>.value(
            buildProductResponseMap(response));
      case '-[InAppPurchasePlugin restoreTransactions:result:]':
        if (restoreException != null) {
          throw restoreException!;
        }
        if (testRestoredError != null) {
          InAppPurchaseIosPlatform.observer
              .restoreCompletedTransactionsFailed(error: testRestoredError!);
          return Future<void>.sync(() {});
        }
        if (!testRestoredTransactionsNull) {
          InAppPurchaseIosPlatform.observer
              .updatedTransactions(transactions: transactions);
        }
        InAppPurchaseIosPlatform.observer
            .paymentQueueRestoreCompletedTransactionsFinished();

        return Future<void>.sync(() {});
      case '-[InAppPurchasePlugin retrieveReceiptData:result:]':
        if (receiptData != null) {
          return Future<void>.value(receiptData);
        } else {
          throw PlatformException(code: 'no_receipt_data');
        }
      case '-[InAppPurchasePlugin refreshReceipt:result:]':
        receiptData = 'refreshed receipt data';
        return Future<void>.sync(() {});
      case '-[InAppPurchasePlugin addPayment:result:]':
        String id = call.arguments['productIdentifier'];
        SKPaymentTransactionWrapper transaction = createPendingTransaction(id);
        InAppPurchaseIosPlatform.observer
            .updatedTransactions(transactions: [transaction]);
        sleep(const Duration(milliseconds: 30));
        if (testTransactionFail) {
          SKPaymentTransactionWrapper transaction_failed =
              createFailedTransaction(id);
          InAppPurchaseIosPlatform.observer
              .updatedTransactions(transactions: [transaction_failed]);
        } else {
          SKPaymentTransactionWrapper transaction_finished =
              createPurchasedTransaction(
                  id, transaction.transactionIdentifier ?? '');
          InAppPurchaseIosPlatform.observer
              .updatedTransactions(transactions: [transaction_finished]);
        }
        break;
      case '-[InAppPurchasePlugin finishTransaction:result:]':
        finishedTransactions.add(createPurchasedTransaction(
            call.arguments["productIdentifier"],
            call.arguments["transactionIdentifier"]));
        break;
      case '-[SKPaymentQueue startObservingTransactionQueue]':
        queueIsActive = true;
        break;
      case '-[SKPaymentQueue stopObservingTransactionQueue]':
        queueIsActive = false;
        break;
    }
    return Future<void>.sync(() {});
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:in_app_purchase_ios/in_app_purchase_ios.dart';
import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import 'fakes/fake_ios_platform.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final FakeIOSPlatform fakeIOSPlatform = FakeIOSPlatform();

  setUpAll(() {
    SystemChannels.platform
        .setMockMethodCallHandler(fakeIOSPlatform.onMethodCall);
  });

  group('present code redemption sheet', () {
    test('null', () async {
      expect(
          await InAppPurchaseIosPlatformAddition().presentCodeRedemptionSheet(),
          null);
    });
  });

  group('refresh receipt data', () {
    test('should refresh receipt data', () async {
      PurchaseVerificationData? receiptData =
          await InAppPurchaseIosPlatformAddition()
              .refreshPurchaseVerificationData();
      expect(receiptData, isNotNull);
      expect(receiptData!.source, kIAPSource);
      expect(receiptData.localVerificationData, 'refreshed receipt data');
      expect(receiptData.serverVerificationData, 'refreshed receipt data');
    });
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter_test/flutter_test.dart';
import 'package:in_app_purchase_ios/in_app_purchase_ios.dart';
import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Can create InAppPurchaseAndroid instance',
      (WidgetTester tester) async {
    InAppPurchaseIosPlatform.registerPlatform();
    final InAppPurchasePlatform androidPlatform =
        InAppPurchasePlatform.instance;
    expect(androidPlatform, isNotNull);
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'package:shared_preferences/shared_preferences.dart';

/// A store of consumable items.
///
/// This is a development prototype tha stores consumables in the shared
/// preferences. Do not use this in real world apps.
class ConsumableStore {
  static const String _kPrefKey = 'consumables';
  static Future<void> _writes = Future.value();

  /// Adds a consumable with ID `id` to the store.
  ///
  /// The consumable is only added after the returned Future is complete.
  static Future<void> save(String id) {
    _writes = _writes.then((void _) => _doSave(id));
    return _writes;
  }

  /// Consumes a consumable with ID `id` from the store.
  ///
  /// The consumable was only consumed after the returned Future is complete.
  static Future<void> consume(String id) {
    _writes = _writes.then((void _) => _doConsume(id));
    return _writes;
  }

  /// Returns the list of consumables from the store.
  static Future<List<String>> load() async {
    return (await SharedPreferences.getInstance()).getStringList(_kPrefKey) ??
        [];
  }

  static Future<void> _doSave(String id) async {
    List<String> cached = await load();
    SharedPreferences prefs = await SharedPreferences.getInstance();
    cached.add(id);
    await prefs.setStringList(_kPrefKey, cached);
  }

  static Future<void> _doConsume(String id) async {
    List<String> cached = await load();
    SharedPreferences prefs = await SharedPreferences.getInstance();
    cached.remove(id);
    await prefs.setStringList(_kPrefKey, cached);
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:in_app_purchase_ios/in_app_purchase_ios.dart';
import 'package:in_app_purchase_ios_example/example_payment_queue_delegate.dart';
import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';
import 'consumable_store.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();

  // When using the Android plugin directly it is mandatory to register
  // the plugin as default instance as part of initializing the app.
  InAppPurchaseIosPlatform.registerPlatform();

  runApp(_MyApp());
}

const bool _kAutoConsume = true;

const String _kConsumableId = 'consumable';
const String _kUpgradeId = 'upgrade';
const String _kSilverSubscriptionId = 'subscription_silver';
const String _kGoldSubscriptionId = 'subscription_gold';
const List<String> _kProductIds = <String>[
  _kConsumableId,
  _kUpgradeId,
  _kSilverSubscriptionId,
  _kGoldSubscriptionId,
];

class _MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<_MyApp> {
  final InAppPurchaseIosPlatform _iapIosPlatform =
      InAppPurchasePlatform.instance as InAppPurchaseIosPlatform;
  final InAppPurchaseIosPlatformAddition _iapIosPlatformAddition =
      InAppPurchasePlatformAddition.instance
          as InAppPurchaseIosPlatformAddition;
  late StreamSubscription<List<PurchaseDetails>> _subscription;
  List<String> _notFoundIds = [];
  List<ProductDetails> _products = [];
  List<PurchaseDetails> _purchases = [];
  List<String> _consumables = [];
  bool _isAvailable = false;
  bool _purchasePending = false;
  bool _loading = true;
  String? _queryProductError;

  @override
  void initState() {
    final Stream<List<PurchaseDetails>> purchaseUpdated =
        _iapIosPlatform.purchaseStream;
    _subscription = purchaseUpdated.listen((purchaseDetailsList) {
      _listenToPurchaseUpdated(purchaseDetailsList);
    }, onDone: () {
      _subscription.cancel();
    }, onError: (error) {
      // handle error here.
    });

    // Register the example payment queue delegate
    _iapIosPlatformAddition.setDelegate(ExamplePaymentQueueDelegate());

    initStoreInfo();
    super.initState();
  }

  Future<void> initStoreInfo() async {
    final bool isAvailable = await _iapIosPlatform.isAvailable();
    if (!isAvailable) {
      setState(() {
        _isAvailable = isAvailable;
        _products = [];
        _purchases = [];
        _notFoundIds = [];
        _consumables = [];
        _purchasePending = false;
        _loading = false;
      });
      return;
    }

    ProductDetailsResponse productDetailResponse =
        await _iapIosPlatform.queryProductDetails(_kProductIds.toSet());
    if (productDetailResponse.error != null) {
      setState(() {
        _queryProductError = productDetailResponse.error!.message;
        _isAvailable = isAvailable;
        _products = productDetailResponse.productDetails;
        _purchases = [];
        _notFoundIds = productDetailResponse.notFoundIDs;
        _consumables = [];
        _purchasePending = false;
        _loading = false;
      });
      return;
    }

    if (productDetailResponse.productDetails.isEmpty) {
      setState(() {
        _queryProductError = null;
        _isAvailable = isAvailable;
        _products = productDetailResponse.productDetails;
        _purchases = [];
        _notFoundIds = productDetailResponse.notFoundIDs;
        _consumables = [];
        _purchasePending = false;
        _loading = false;
      });
      return;
    }

    List<String> consumables = await ConsumableStore.load();
    setState(() {
      _isAvailable = isAvailable;
      _products = productDetailResponse.productDetails;
      _notFoundIds = productDetailResponse.notFoundIDs;
      _consumables = consumables;
      _purchasePending = false;
      _loading = false;
    });
  }

  @override
  void dispose() {
    _subscription.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    List<Widget> stack = [];
    if (_queryProductError == null) {
      stack.add(
        ListView(
          children: [
            _buildConnectionCheckTile(),
            _buildProductList(),
            _buildConsumableBox(),
            _buildRestoreButton(),
          ],
        ),
      );
    } else {
      stack.add(Center(
        child: Text(_queryProductError!),
      ));
    }
    if (_purchasePending) {
      stack.add(
        Stack(
          children: [
            Opacity(
              opacity: 0.3,
              child: const ModalBarrier(dismissible: false, color: Colors.grey),
            ),
            Center(
              child: CircularProgressIndicator(),
            ),
          ],
        ),
      );
    }

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('IAP Example'),
        ),
        body: Stack(
          children: stack,
        ),
      ),
    );
  }

  Card _buildConnectionCheckTile() {
    if (_loading) {
      return Card(child: ListTile(title: const Text('Trying to connect...')));
    }
    final Widget storeHeader = ListTile(
      leading: Icon(_isAvailable ? Icons.check : Icons.block,
          color: _isAvailable ? Colors.green : ThemeData.light().errorColor),
      title: Text(
          'The store is ' + (_isAvailable ? 'available' : 'unavailable') + '.'),
    );
    final List<Widget> children = <Widget>[storeHeader];

    if (!_isAvailable) {
      children.addAll([
        Divider(),
        ListTile(
          title: Text('Not connected',
              style: TextStyle(color: ThemeData.light().errorColor)),
          subtitle: const Text(
              'Unable to connect to the payments processor. Has this app been configured correctly? See the example README for instructions.'),
        ),
      ]);
    }
    return Card(child: Column(children: children));
  }

  Card _buildProductList() {
    if (_loading) {
      return Card(
          child: (ListTile(
              leading: CircularProgressIndicator(),
              title: Text('Fetching products...'))));
    }
    if (!_isAvailable) {
      return Card();
    }
    final ListTile productHeader = ListTile(title: Text('Products for Sale'));
    List<ListTile> productList = <ListTile>[];
    if (_notFoundIds.isNotEmpty) {
      productList.add(ListTile(
          title: Text('[${_notFoundIds.join(", ")}] not found',
              style: TextStyle(color: ThemeData.light().errorColor)),
          subtitle: Text(
              'This app needs special configuration to run. Please see example/README.md for instructions.')));
    }

    // This loading previous purchases code is just a demo. Please do not use this as it is.
    // In your app you should always verify the purchase data using the `verificationData` inside the [PurchaseDetails] object before trusting it.
    // We recommend that you use your own server to verify the purchase data.
    Map<String, PurchaseDetails> purchases =
        Map.fromEntries(_purchases.map((PurchaseDetails purchase) {
      if (purchase.pendingCompletePurchase) {
        _iapIosPlatform.completePurchase(purchase);
      }
      return MapEntry<String, PurchaseDetails>(purchase.productID, purchase);
    }));
    productList.addAll(_products.map(
      (ProductDetails productDetails) {
        PurchaseDetails? previousPurchase = purchases[productDetails.id];
        return ListTile(
            title: Text(
              productDetails.title,
            ),
            subtitle: Text(
              productDetails.description,
            ),
            trailing: previousPurchase != null
                ? IconButton(
                    onPressed: () {
                      _iapIosPlatformAddition.showPriceConsentIfNeeded();
                    },
                    icon: Icon(Icons.upgrade))
                : TextButton(
                    child: Text(productDetails.price),
                    style: TextButton.styleFrom(
                      backgroundColor: Colors.green[800],
                      primary: Colors.white,
                    ),
                    onPressed: () {
                      PurchaseParam purchaseParam = PurchaseParam(
                        productDetails: productDetails,
                        applicationUserName: null,
                      );
                      if (productDetails.id == _kConsumableId) {
                        _iapIosPlatform.buyConsumable(
                            purchaseParam: purchaseParam,
                            autoConsume: _kAutoConsume || Platform.isIOS);
                      } else {
                        _iapIosPlatform.buyNonConsumable(
                            purchaseParam: purchaseParam);
                      }
                    },
                  ));
      },
    ));

    return Card(
        child:
            Column(children: <Widget>[productHeader, Divider()] + productList));
  }

  Card _buildConsumableBox() {
    if (_loading) {
      return Card(
          child: (ListTile(
              leading: CircularProgressIndicator(),
              title: Text('Fetching consumables...'))));
    }
    if (!_isAvailable || _notFoundIds.contains(_kConsumableId)) {
      return Card();
    }
    final ListTile consumableHeader =
        ListTile(title: Text('Purchased consumables'));
    final List<Widget> tokens = _consumables.map((String id) {
      return GridTile(
        child: IconButton(
          icon: Icon(
            Icons.stars,
            size: 42.0,
            color: Colors.orange,
          ),
          splashColor: Colors.yellowAccent,
          onPressed: () => consume(id),
        ),
      );
    }).toList();
    return Card(
        child: Column(children: <Widget>[
      consumableHeader,
      Divider(),
      GridView.count(
        crossAxisCount: 5,
        children: tokens,
        shrinkWrap: true,
        padding: EdgeInsets.all(16.0),
      )
    ]));
  }

  Widget _buildRestoreButton() {
    if (_loading) {
      return Container();
    }

    return Padding(
      padding: const EdgeInsets.all(4.0),
      child: Row(
        mainAxisSize: MainAxisSize.max,
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          TextButton(
            child: Text('Restore purchases'),
            style: TextButton.styleFrom(
              backgroundColor: Theme.of(context).primaryColor,
              primary: Colors.white,
            ),
            onPressed: () => _iapIosPlatform.restorePurchases(),
          ),
        ],
      ),
    );
  }

  Future<void> consume(String id) async {
    await ConsumableStore.consume(id);
    final List<String> consumables = await ConsumableStore.load();
    setState(() {
      _consumables = consumables;
    });
  }

  void showPendingUI() {
    setState(() {
      _purchasePending = true;
    });
  }

  void deliverProduct(PurchaseDetails purchaseDetails) async {
    // IMPORTANT!! Always verify purchase details before delivering the product.
    if (purchaseDetails.productID == _kConsumableId) {
      await ConsumableStore.save(purchaseDetails.purchaseID!);
      List<String> consumables = await ConsumableStore.load();
      setState(() {
        _purchasePending = false;
        _consumables = consumables;
      });
    } else {
      setState(() {
        _purchases.add(purchaseDetails);
        _purchasePending = false;
      });
    }
  }

  void handleError(IAPError error) {
    setState(() {
      _purchasePending = false;
    });
  }

  Future<bool> _verifyPurchase(PurchaseDetails purchaseDetails) {
    // IMPORTANT!! Always verify a purchase before delivering the product.
    // For the purpose of an example, we directly return true.
    return Future<bool>.value(true);
  }

  void _handleInvalidPurchase(PurchaseDetails purchaseDetails) {
    // handle invalid purchase here if  _verifyPurchase` failed.
  }

  void _listenToPurchaseUpdated(List<PurchaseDetails> purchaseDetailsList) {
    purchaseDetailsList.forEach((PurchaseDetails purchaseDetails) async {
      if (purchaseDetails.status == PurchaseStatus.pending) {
        showPendingUI();
      } else {
        if (purchaseDetails.status == PurchaseStatus.error) {
          handleError(purchaseDetails.error!);
        } else if (purchaseDetails.status == PurchaseStatus.purchased ||
            purchaseDetails.status == PurchaseStatus.restored) {
          bool valid = await _verifyPurchase(purchaseDetails);
          if (valid) {
            deliverProduct(purchaseDetails);
          } else {
            _handleInvalidPurchase(purchaseDetails);
            return;
          }
        }

        if (purchaseDetails.pendingCompletePurchase) {
          await _iapIosPlatform.completePurchase(purchaseDetails);
        }
      }
    });
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_ios/store_kit_wrappers.dart';

/// Example implementation of the
/// [`SKPaymentQueueDelegate`](https://developer.apple.com/documentation/storekit/skpaymentqueuedelegate?language=objc).
///
/// The payment queue delegate can be implementated to provide information
/// needed to complete transactions.
class ExamplePaymentQueueDelegate implements SKPaymentQueueDelegateWrapper {
  @override
  bool shouldContinueTransaction(
      SKPaymentTransactionWrapper transaction, SKStorefrontWrapper storefront) {
    return true;
  }

  @override
  bool shouldShowPriceConsent() {
    return false;
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

export 'src/in_app_purchase_ios_platform.dart';
export 'src/in_app_purchase_ios_platform_addition.dart';
export 'src/types/types.dart';
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

export 'src/store_kit_wrappers/sk_payment_queue_delegate_wrapper.dart';
export 'src/store_kit_wrappers/sk_payment_queue_wrapper.dart';
export 'src/store_kit_wrappers/sk_payment_transaction_wrappers.dart';
export 'src/store_kit_wrappers/sk_product_wrapper.dart';
export 'src/store_kit_wrappers/sk_receipt_manager.dart';
export 'src/store_kit_wrappers/sk_request_maker.dart';
export 'src/store_kit_wrappers/sk_storefront_wrapper.dart';
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import '../../store_kit_wrappers.dart';

/// The class represents the information of a product as registered in the Apple
/// AppStore.
class AppStoreProductDetails extends ProductDetails {
  /// Creates a new AppStore specific product details object with the provided
  /// details.
  AppStoreProductDetails({
    required String id,
    required String title,
    required String description,
    required String price,
    required double rawPrice,
    required String currencyCode,
    required this.skProduct,
    required String currencySymbol,
  }) : super(
          id: id,
          title: title,
          description: description,
          price: price,
          rawPrice: rawPrice,
          currencyCode: currencyCode,
          currencySymbol: currencySymbol,
        );

  /// Points back to the [SKProductWrapper] object that was used to generate
  /// this [AppStoreProductDetails] object.
  final SKProductWrapper skProduct;

  /// Generate a [AppStoreProductDetails] object based on an iOS [SKProductWrapper] object.
  factory AppStoreProductDetails.fromSKProduct(SKProductWrapper product) {
    return AppStoreProductDetails(
      id: product.productIdentifier,
      title: product.localizedTitle,
      description: product.localizedDescription,
      price: product.priceLocale.currencySymbol + product.price,
      rawPrice: double.parse(product.price),
      currencyCode: product.priceLocale.currencyCode,
      currencySymbol: product.priceLocale.currencySymbol.isNotEmpty
          ? product.priceLocale.currencySymbol
          : product.priceLocale.currencyCode,
      skProduct: product,
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import '../../store_kit_wrappers.dart';

/// Apple AppStore specific parameter object for generating a purchase.
class AppStorePurchaseParam extends PurchaseParam {
  /// Creates a new [AppStorePurchaseParam] object with the given data.
  AppStorePurchaseParam({
    required ProductDetails productDetails,
    String? applicationUserName,
    this.simulatesAskToBuyInSandbox = false,
  }) : super(
          productDetails: productDetails,
          applicationUserName: applicationUserName,
        );

  /// Set it to `true` to produce an "ask to buy" flow for this payment in the
  /// sandbox.
  ///
  /// If you want to test [simulatesAskToBuyInSandbox], you should ensure that
  /// you create an instance of the [AppStorePurchaseParam] class and set its
  /// [simulateAskToBuyInSandbox] field to `true` and use it with the
  /// `buyNonConsumable` or `buyConsumable` methods.
  ///
  /// See also [SKPaymentWrapper.simulatesAskToBuyInSandbox].
  final bool simulatesAskToBuyInSandbox;
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import '../../in_app_purchase_ios.dart';
import '../../store_kit_wrappers.dart';
import '../store_kit_wrappers/enum_converters.dart';

/// The class represents the information of a purchase made with the Apple
/// AppStore.
class AppStorePurchaseDetails extends PurchaseDetails {
  /// Creates a new AppStore specific purchase details object with the provided
  /// details.
  AppStorePurchaseDetails(
      {String? purchaseID,
      required String productID,
      required PurchaseVerificationData verificationData,
      required String? transactionDate,
      required this.skPaymentTransaction,
      required PurchaseStatus status})
      : super(
            productID: productID,
            purchaseID: purchaseID,
            transactionDate: transactionDate,
            verificationData: verificationData,
            status: status) {
    this.status = status;
  }

  /// Points back to the [SKPaymentTransactionWrapper] which was used to
  /// generate this [AppStorePurchaseDetails] object.
  final SKPaymentTransactionWrapper skPaymentTransaction;

  late PurchaseStatus _status;

  /// The status that this [PurchaseDetails] is currently on.
  PurchaseStatus get status => _status;
  set status(PurchaseStatus status) {
    _pendingCompletePurchase = status != PurchaseStatus.pending;
    _status = status;
  }

  bool _pendingCompletePurchase = false;
  bool get pendingCompletePurchase => _pendingCompletePurchase;

  /// Generate a [AppStorePurchaseDetails] object based on an iOS
  /// [SKPaymentTransactionWrapper] object.
  factory AppStorePurchaseDetails.fromSKTransaction(
    SKPaymentTransactionWrapper transaction,
    String base64EncodedReceipt,
  ) {
    final AppStorePurchaseDetails purchaseDetails = AppStorePurchaseDetails(
      productID: transaction.payment.productIdentifier,
      purchaseID: transaction.transactionIdentifier,
      skPaymentTransaction: transaction,
      status: SKTransactionStatusConverter()
          .toPurchaseStatus(transaction.transactionState),
      transactionDate: transaction.transactionTimeStamp != null
          ? (transaction.transactionTimeStamp! * 1000).toInt().toString()
          : null,
      verificationData: PurchaseVerificationData(
          localVerificationData: base64EncodedReceipt,
          serverVerificationData: base64EncodedReceipt,
          source: kIAPSource),
    );

    if (purchaseDetails.status == PurchaseStatus.error) {
      purchaseDetails.error = IAPError(
        source: kIAPSource,
        code: kPurchaseErrorCode,
        message: transaction.error?.domain ?? '',
        details: transaction.error?.userInfo,
      );
    }

    return purchaseDetails;
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
export 'app_store_product_details.dart';
export 'app_store_purchase_details.dart';
export 'app_store_purchase_param.dart';
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:ui' show hashValues;

import 'package:json_annotation/json_annotation.dart';

import 'enum_converters.dart';
import 'sk_payment_queue_wrapper.dart';
import 'sk_product_wrapper.dart';

part 'sk_payment_transaction_wrappers.g.dart';

/// Callback handlers for transaction status changes.
///
/// Must be subclassed. Must be instantiated and added to the
/// [SKPaymentQueueWrapper] via [SKPaymentQueueWrapper.setTransactionObserver]
/// at app launch.
///
/// This class is a Dart wrapper around [SKTransactionObserver](https://developer.apple.com/documentation/storekit/skpaymenttransactionobserver?language=objc).
abstract class SKTransactionObserverWrapper {
  /// Triggered when any transactions are updated.
  void updatedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions});

  /// Triggered when any transactions are removed from the payment queue.
  void removedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions});

  /// Triggered when there is an error while restoring transactions.
  void restoreCompletedTransactionsFailed({required SKError error});

  /// Triggered when payment queue has finished sending restored transactions.
  void paymentQueueRestoreCompletedTransactionsFinished();

  /// Triggered when a user initiates an in-app purchase from App Store.
  ///
  /// Return `true` to continue the transaction in your app. If you have
  /// multiple [SKTransactionObserverWrapper]s, the transaction will continue if
  /// any [SKTransactionObserverWrapper] returns `true`. Return `false` to defer
  /// or cancel the transaction. For example, you may need to defer a
  /// transaction if the user is in the middle of onboarding. You can also
  /// continue the transaction later by calling [addPayment] with the
  /// `payment` param from this method.
  bool shouldAddStorePayment(
      {required SKPaymentWrapper payment, required SKProductWrapper product});
}

/// The state of a transaction.
///
/// Dart wrapper around StoreKit's
/// [SKPaymentTransactionState](https://developer.apple.com/documentation/storekit/skpaymenttransactionstate?language=objc).
enum SKPaymentTransactionStateWrapper {
  /// Indicates the transaction is being processed in App Store.
  ///
  /// You should update your UI to indicate that you are waiting for the
  /// transaction to update to another state. Never complete a transaction that
  /// is still in a purchasing state.
  @JsonValue(0)
  purchasing,

  /// The user's payment has been succesfully processed.
  ///
  /// You should provide the user the content that they purchased.
  @JsonValue(1)
  purchased,

  /// The transaction failed.
  ///
  /// Check the [SKPaymentTransactionWrapper.error] property from
  /// [SKPaymentTransactionWrapper] for details.
  @JsonValue(2)
  failed,

  /// This transaction is restoring content previously purchased by the user.
  ///
  /// The previous transaction information can be obtained in
  /// [SKPaymentTransactionWrapper.originalTransaction] from
  /// [SKPaymentTransactionWrapper].
  @JsonValue(3)
  restored,

  /// The transaction is in the queue but pending external action. Wait for
  /// another callback to get the final state.
  ///
  /// You should update your UI to indicate that you are waiting for the
  /// transaction to update to another state.
  @JsonValue(4)
  deferred,

  /// Indicates the transaction is in an unspecified state.
  @JsonValue(-1)
  unspecified,
}

/// Created when a payment is added to the [SKPaymentQueueWrapper].
///
/// Transactions are delivered to your app when a payment is finished
/// processing. Completed transactions provide a receipt and a transaction
/// identifier that the app can use to save a permanent record of the processed
/// payment.
///
/// Dart wrapper around StoreKit's
/// [SKPaymentTransaction](https://developer.apple.com/documentation/storekit/skpaymenttransaction?language=objc).
@JsonSerializable(createToJson: true)
class SKPaymentTransactionWrapper {
  /// Creates a new [SKPaymentTransactionWrapper] with the provided information.
  SKPaymentTransactionWrapper({
    required this.payment,
    required this.transactionState,
    this.originalTransaction,
    this.transactionTimeStamp,
    this.transactionIdentifier,
    this.error,
  });

  /// Constructs an instance of this from a key value map of data.
  ///
  /// The map needs to have named string keys with values matching the names and
  /// types of all of the members on this class. The `map` parameter must not be
  /// null.
  factory SKPaymentTransactionWrapper.fromJson(Map<String, dynamic> map) {
    return _$SKPaymentTransactionWrapperFromJson(map);
  }

  /// Current transaction state.
  @SKTransactionStatusConverter()
  final SKPaymentTransactionStateWrapper transactionState;

  /// The payment that has been created and added to the payment queue which
  /// generated this transaction.
  final SKPaymentWrapper payment;

  /// The original Transaction.
  ///
  /// Only available if the [transactionState] is [SKPaymentTransactionStateWrapper.restored].
  /// Otherwise the value is `null`.
  ///
  /// When the [transactionState]
  /// is [SKPaymentTransactionStateWrapper.restored], the current transaction
  /// object holds a new [transactionIdentifier].
  final SKPaymentTransactionWrapper? originalTransaction;

  /// The timestamp of the transaction.
  ///
  /// Seconds since epoch. It is only defined when the [transactionState] is
  /// [SKPaymentTransactionStateWrapper.purchased] or
  /// [SKPaymentTransactionStateWrapper.restored].
  /// Otherwise, the value is `null`.
  final double? transactionTimeStamp;

  /// The unique string identifer of the transaction.
  ///
  /// It is only defined when the [transactionState] is
  /// [SKPaymentTransactionStateWrapper.purchased] or
  /// [SKPaymentTransactionStateWrapper.restored]. You may wish to record this
  /// string as part of an audit trail for App Store purchases. The value of
  /// this string corresponds to the same property in the receipt.
  ///
  /// The value is `null` if it is an unsuccessful transaction.
  final String? transactionIdentifier;

  /// The error object
  ///
  /// Only available if the [transactionState] is
  /// [SKPaymentTransactionStateWrapper.failed].
  final SKError? error;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKPaymentTransactionWrapper typedOther =
        other as SKPaymentTransactionWrapper;
    return typedOther.payment == payment &&
        typedOther.transactionState == transactionState &&
        typedOther.originalTransaction == originalTransaction &&
        typedOther.transactionTimeStamp == transactionTimeStamp &&
        typedOther.transactionIdentifier == transactionIdentifier &&
        typedOther.error == error;
  }

  @override
  int get hashCode => hashValues(
      this.payment,
      this.transactionState,
      this.originalTransaction,
      this.transactionTimeStamp,
      this.transactionIdentifier,
      this.error);

  @override
  String toString() => _$SKPaymentTransactionWrapperToJson(this).toString();

  /// The payload that is used to finish this transaction.
  Map<String, String?> toFinishMap() => <String, String?>{
        "transactionIdentifier": this.transactionIdentifier,
        "productIdentifier": this.payment.productIdentifier,
      };
}
```

```dart
// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sk_payment_transaction_wrappers.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SKPaymentTransactionWrapper _$SKPaymentTransactionWrapperFromJson(Map json) =>
    SKPaymentTransactionWrapper(
      payment: SKPaymentWrapper.fromJson(
          Map<String, dynamic>.from(json['payment'] as Map)),
      transactionState: const SKTransactionStatusConverter()
          .fromJson(json['transactionState'] as int?),
      originalTransaction: json['originalTransaction'] == null
          ? null
          : SKPaymentTransactionWrapper.fromJson(
              Map<String, dynamic>.from(json['originalTransaction'] as Map)),
      transactionTimeStamp: (json['transactionTimeStamp'] as num?)?.toDouble(),
      transactionIdentifier: json['transactionIdentifier'] as String?,
      error: json['error'] == null
          ? null
          : SKError.fromJson(Map<String, dynamic>.from(json['error'] as Map)),
    );

Map<String, dynamic> _$SKPaymentTransactionWrapperToJson(
        SKPaymentTransactionWrapper instance) =>
    <String, dynamic>{
      'transactionState': const SKTransactionStatusConverter()
          .toJson(instance.transactionState),
      'payment': instance.payment,
      'originalTransaction': instance.originalTransaction,
      'transactionTimeStamp': instance.transactionTimeStamp,
      'transactionIdentifier': instance.transactionIdentifier,
      'error': instance.error,
    };
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import '../channel.dart';

///This class contains static methods to manage StoreKit receipts.
class SKReceiptManager {
  /// Retrieve the receipt data from your application's main bundle.
  ///
  /// The receipt data will be based64 encoded. The structure of the payload is defined using ASN.1.
  /// You can use the receipt data retrieved by this method to validate users' purchases.
  /// There are 2 ways to do so. Either validate locally or validate with App Store.
  /// For more details on how to validate the receipt data, you can refer to Apple's document about [`About Receipt Validation`](https://developer.apple.com/library/archive/releasenotes/General/ValidateAppStoreReceipt/Introduction.html#//apple_ref/doc/uid/TP40010573-CH105-SW1).
  /// If the receipt is invalid or missing, you can use [SKRequestMaker.startRefreshReceiptRequest] to request a new receipt.
  static Future<String> retrieveReceiptData() async {
    return (await channel.invokeMethod<String>(
            '-[InAppPurchasePlugin retrieveReceiptData:result:]')) ??
        '';
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';
import 'package:json_annotation/json_annotation.dart';

import '../../store_kit_wrappers.dart';

part 'enum_converters.g.dart';

/// Serializer for [SKPaymentTransactionStateWrapper].
///
/// Use these in `@JsonSerializable()` classes by annotating them with
/// `@SKTransactionStatusConverter()`.
class SKTransactionStatusConverter
    implements JsonConverter<SKPaymentTransactionStateWrapper, int?> {
  /// Default const constructor.
  const SKTransactionStatusConverter();

  @override
  SKPaymentTransactionStateWrapper fromJson(int? json) {
    if (json == null) {
      return SKPaymentTransactionStateWrapper.unspecified;
    }
    return $enumDecode<SKPaymentTransactionStateWrapper, dynamic>(
        _$SKPaymentTransactionStateWrapperEnumMap
            .cast<SKPaymentTransactionStateWrapper, dynamic>(),
        json);
  }

  /// Converts an [SKPaymentTransactionStateWrapper] to a [PurchaseStatus].
  PurchaseStatus toPurchaseStatus(SKPaymentTransactionStateWrapper object) {
    switch (object) {
      case SKPaymentTransactionStateWrapper.purchasing:
      case SKPaymentTransactionStateWrapper.deferred:
        return PurchaseStatus.pending;
      case SKPaymentTransactionStateWrapper.purchased:
        return PurchaseStatus.purchased;
      case SKPaymentTransactionStateWrapper.restored:
        return PurchaseStatus.restored;
      case SKPaymentTransactionStateWrapper.failed:
      case SKPaymentTransactionStateWrapper.unspecified:
        return PurchaseStatus.error;
    }
  }

  @override
  int toJson(SKPaymentTransactionStateWrapper object) =>
      _$SKPaymentTransactionStateWrapperEnumMap[object]!;
}

/// Serializer for [SKSubscriptionPeriodUnit].
///
/// Use these in `@JsonSerializable()` classes by annotating them with
/// `@SKSubscriptionPeriodUnitConverter()`.
class SKSubscriptionPeriodUnitConverter
    implements JsonConverter<SKSubscriptionPeriodUnit, int?> {
  /// Default const constructor.
  const SKSubscriptionPeriodUnitConverter();

  @override
  SKSubscriptionPeriodUnit fromJson(int? json) {
    if (json == null) {
      return SKSubscriptionPeriodUnit.day;
    }
    return $enumDecode<SKSubscriptionPeriodUnit, dynamic>(
        _$SKSubscriptionPeriodUnitEnumMap
            .cast<SKSubscriptionPeriodUnit, dynamic>(),
        json);
  }

  @override
  int toJson(SKSubscriptionPeriodUnit object) =>
      _$SKSubscriptionPeriodUnitEnumMap[object]!;
}

/// Serializer for [SKProductDiscountPaymentMode].
///
/// Use these in `@JsonSerializable()` classes by annotating them with
/// `@SKProductDiscountPaymentModeConverter()`.
class SKProductDiscountPaymentModeConverter
    implements JsonConverter<SKProductDiscountPaymentMode, int?> {
  /// Default const constructor.
  const SKProductDiscountPaymentModeConverter();

  @override
  SKProductDiscountPaymentMode fromJson(int? json) {
    if (json == null) {
      return SKProductDiscountPaymentMode.payAsYouGo;
    }
    return $enumDecode<SKProductDiscountPaymentMode, dynamic>(
        _$SKProductDiscountPaymentModeEnumMap
            .cast<SKProductDiscountPaymentMode, dynamic>(),
        json);
  }

  @override
  int toJson(SKProductDiscountPaymentMode object) =>
      _$SKProductDiscountPaymentModeEnumMap[object]!;
}

// Define a class so we generate serializer helper methods for the enums
// See https://github.com/google/json_serializable.dart/issues/778
@JsonSerializable()
class _SerializedEnums {
  late SKPaymentTransactionStateWrapper response;
  late SKSubscriptionPeriodUnit unit;
  late SKProductDiscountPaymentMode discountPaymentMode;
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:ui' show hashValues;

import 'package:json_annotation/json_annotation.dart';

part 'sk_storefront_wrapper.g.dart';

/// Contains the location and unique identifier of an Apple App Store storefront.
///
/// Dart wrapper around StoreKit's
/// [SKStorefront](https://developer.apple.com/documentation/storekit/skstorefront?language=objc).
@JsonSerializable(createToJson: true)
class SKStorefrontWrapper {
  /// Creates a new [SKStorefrontWrapper] with the provided information.
  SKStorefrontWrapper({
    required this.countryCode,
    required this.identifier,
  });

  /// Constructs an instance of the [SKStorefrontWrapper] from a key value map
  /// of data.
  ///
  /// The map needs to have named string keys with values matching the names and
  /// types of all of the members on this class. The `map` parameter must not be
  /// null.
  factory SKStorefrontWrapper.fromJson(Map<String, dynamic> map) {
    return _$SKStorefrontWrapperFromJson(map);
  }

  /// The three-letter code representing the country or region associated with
  /// the App Store storefront.
  final String countryCode;

  /// A value defined by Apple that uniquely identifies an App Store storefront.
  final String identifier;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKStorefrontWrapper typedOther = other as SKStorefrontWrapper;
    return typedOther.countryCode == countryCode &&
        typedOther.identifier == identifier;
  }

  @override
  int get hashCode => hashValues(
        this.countryCode,
        this.identifier,
      );

  @override
  String toString() => _$SKStorefrontWrapperToJson(this).toString();

  /// Converts the instance to a key value map which can be used to serialize
  /// to JSON format.
  Map<String, dynamic> toMap() => _$SKStorefrontWrapperToJson(this);
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'package:flutter/services.dart';

import '../channel.dart';
import 'sk_product_wrapper.dart';

/// A request maker that handles all the requests made by SKRequest subclasses.
///
/// There are multiple [SKRequest](https://developer.apple.com/documentation/storekit/skrequest?language=objc) subclasses handling different requests in the `StoreKit` with multiple delegate methods,
/// we consolidated all the `SKRequest` subclasses into this class to make requests in a more straightforward way.
/// The request maker will create a SKRequest object, immediately starting it, and completing the future successfully or throw an exception depending on what happened to the request.
class SKRequestMaker {
  /// Fetches product information for a list of given product identifiers.
  ///
  /// The `productIdentifiers` should contain legitimate product identifiers that you declared for the products in the iTunes Connect. Invalid identifiers
  /// will be stored and returned in [SkProductResponseWrapper.invalidProductIdentifiers]. Duplicate values in `productIdentifiers` will be omitted.
  /// If `productIdentifiers` is null, an `storekit_invalid_argument` error will be returned. If `productIdentifiers` is empty, a [SkProductResponseWrapper]
  /// will still be returned with [SkProductResponseWrapper.products] being null.
  ///
  /// [SkProductResponseWrapper] is returned if there is no error during the request.
  /// A [PlatformException] is thrown if the platform code making the request fails.
  Future<SkProductResponseWrapper> startProductRequest(
      List<String> productIdentifiers) async {
    final Map<String, dynamic>? productResponseMap =
        await channel.invokeMapMethod<String, dynamic>(
      '-[InAppPurchasePlugin startProductRequest:result:]',
      productIdentifiers,
    );
    if (productResponseMap == null) {
      throw PlatformException(
        code: 'storekit_no_response',
        message: 'StoreKit: Failed to get response from platform.',
      );
    }
    return SkProductResponseWrapper.fromJson(productResponseMap);
  }

  /// Uses [SKReceiptRefreshRequest](https://developer.apple.com/documentation/storekit/skreceiptrefreshrequest?language=objc) to request a new receipt.
  ///
  /// If the receipt is invalid or missing, you can use this API to request a new receipt.
  /// The [receiptProperties] is optional and it exists only for [sandbox testing](https://developer.apple.com/apple-pay/sandbox-testing/). In the production app, call this API without pass in the [receiptProperties] parameter.
  /// To test in the sandbox, you can request a receipt with any combination of properties to test the state transitions related to [`Volume Purchase Plan`](https://www.apple.com/business/site/docs/VPP_Business_Guide.pdf) receipts.
  /// The valid keys in the receiptProperties are below (All of them are of type bool):
  /// * isExpired: whether the receipt is expired.
  /// * isRevoked: whether the receipt has been revoked.
  /// * isVolumePurchase: whether the receipt is a Volume Purchase Plan receipt.
  Future<void> startRefreshReceiptRequest(
      {Map<String, dynamic>? receiptProperties}) {
    return channel.invokeMethod<void>(
      '-[InAppPurchasePlugin refreshReceipt:result:]',
      receiptProperties,
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:ui' show hashValues;

import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:in_app_purchase_ios/store_kit_wrappers.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:meta/meta.dart';

import '../channel.dart';
import '../in_app_purchase_ios_platform.dart';
import 'sk_payment_queue_delegate_wrapper.dart';
import 'sk_payment_transaction_wrappers.dart';
import 'sk_product_wrapper.dart';

part 'sk_payment_queue_wrapper.g.dart';

/// A wrapper around
/// [`SKPaymentQueue`](https://developer.apple.com/documentation/storekit/skpaymentqueue?language=objc).
///
/// The payment queue contains payment related operations. It communicates with
/// the App Store and presents a user interface for the user to process and
/// authorize payments.
///
/// Full information on using `SKPaymentQueue` and processing purchases is
/// available at the [In-App Purchase Programming
/// Guide](https://developer.apple.com/library/archive/documentation/NetworkingInternet/Conceptual/StoreKitGuide/Introduction.html#//apple_ref/doc/uid/TP40008267).
class SKPaymentQueueWrapper {
  /// Returns the default payment queue.
  ///
  /// We do not support instantiating a custom payment queue, hence the
  /// singleton. However, you can override the observer.
  factory SKPaymentQueueWrapper() {
    return _singleton;
  }

  SKPaymentQueueWrapper._();

  static final SKPaymentQueueWrapper _singleton = SKPaymentQueueWrapper._();

  SKPaymentQueueDelegateWrapper? _paymentQueueDelegate;
  SKTransactionObserverWrapper? _observer;

  /// Calls [`-[SKPaymentQueue transactions]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/1506026-transactions?language=objc)
  Future<List<SKPaymentTransactionWrapper>> transactions() async {
    return _getTransactionList((await channel
        .invokeListMethod<dynamic>('-[SKPaymentQueue transactions]'))!);
  }

  /// Calls [`-[SKPaymentQueue canMakePayments:]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/1506139-canmakepayments?language=objc).
  static Future<bool> canMakePayments() async =>
      (await channel
          .invokeMethod<bool>('-[SKPaymentQueue canMakePayments:]')) ??
      false;

  /// Sets an observer to listen to all incoming transaction events.
  ///
  /// This should be called and set as soon as the app launches in order to
  /// avoid missing any purchase updates from the App Store. See the
  /// documentation on StoreKit's [`-[SKPaymentQueue
  /// addTransactionObserver:]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/1506042-addtransactionobserver?language=objc).
  void setTransactionObserver(SKTransactionObserverWrapper observer) {
    _observer = observer;
    channel.setMethodCallHandler(handleObserverCallbacks);
  }

  /// Instructs the iOS implementation to register a transaction observer and
  /// start listening to it.
  ///
  /// Call this method when the first listener is subscribed to the
  /// [InAppPurchaseIosPlatform.purchaseStream].
  Future startObservingTransactionQueue() => channel
      .invokeMethod<void>('-[SKPaymentQueue startObservingTransactionQueue]');

  /// Instructs the iOS implementation to remove the transaction observer and
  /// stop listening to it.
  ///
  /// Call this when there are no longer any listeners subscribed to the
  /// [InAppPurchaseIosPlatform.purchaseStream].
  Future stopObservingTransactionQueue() => channel
      .invokeMethod<void>('-[SKPaymentQueue stopObservingTransactionQueue]');

  /// Sets an implementation of the [SKPaymentQueueDelegateWrapper].
  ///
  /// The [SKPaymentQueueDelegateWrapper] can be used to inform iOS how to
  /// finish transactions when the storefront changes or if the price consent
  /// sheet should be displayed when the price of a subscription has changed. If
  /// no delegate is registered iOS will fallback to it's default configuration.
  /// See the documentation on StoreKite's [`-[SKPaymentQueue delegate:]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/3182429-delegate?language=objc).
  ///
  /// When set to `null` the payment queue delegate will be removed and the
  /// default behaviour will apply (see [documentation](https://developer.apple.com/documentation/storekit/skpaymentqueue/3182429-delegate?language=objc)).
  Future setDelegate(SKPaymentQueueDelegateWrapper? delegate) async {
    if (delegate == null) {
      await channel.invokeMethod<void>('-[SKPaymentQueue removeDelegate]');
      paymentQueueDelegateChannel.setMethodCallHandler(null);
    } else {
      await channel.invokeMethod<void>('-[SKPaymentQueue registerDelegate]');
      paymentQueueDelegateChannel
          .setMethodCallHandler(handlePaymentQueueDelegateCallbacks);
    }

    _paymentQueueDelegate = delegate;
  }

  /// Posts a payment to the queue.
  ///
  /// This sends a purchase request to the App Store for confirmation.
  /// Transaction updates will be delivered to the set
  /// [SkTransactionObserverWrapper].
  ///
  /// A couple preconditions need to be met before calling this method.
  ///
  ///   - At least one [SKTransactionObserverWrapper] should have been added to
  ///     the payment queue using [addTransactionObserver].
  ///   - The [payment.productIdentifier] needs to have been previously fetched
  ///     using [SKRequestMaker.startProductRequest] so that a valid `SKProduct`
  ///     has been cached in the platform side already. Because of this
  ///     [payment.productIdentifier] cannot be hardcoded.
  ///
  /// This method calls StoreKit's [`-[SKPaymentQueue addPayment:]`]
  /// (https://developer.apple.com/documentation/storekit/skpaymentqueue/1506036-addpayment?preferredLanguage=occ).
  ///
  /// Also see [sandbox
  /// testing](https://developer.apple.com/apple-pay/sandbox-testing/).
  Future<void> addPayment(SKPaymentWrapper payment) async {
    assert(_observer != null,
        '[in_app_purchase]: Trying to add a payment without an observer. One must be set using `SkPaymentQueueWrapper.setTransactionObserver` before the app launches.');
    final Map<String, dynamic> requestMap = payment.toMap();
    await channel.invokeMethod<void>(
      '-[InAppPurchasePlugin addPayment:result:]',
      requestMap,
    );
  }

  /// Finishes a transaction and removes it from the queue.
  ///
  /// This method should be called after the given [transaction] has been
  /// succesfully processed and its content has been delivered to the user.
  /// Transaction status updates are propagated to [SkTransactionObserver].
  ///
  /// This will throw a Platform exception if [transaction.transactionState] is
  /// [SKPaymentTransactionStateWrapper.purchasing].
  ///
  /// This method calls StoreKit's [`-[SKPaymentQueue
  /// finishTransaction:]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/1506003-finishtransaction?language=objc).
  Future<void> finishTransaction(
      SKPaymentTransactionWrapper transaction) async {
    Map<String, String?> requestMap = transaction.toFinishMap();
    await channel.invokeMethod<void>(
      '-[InAppPurchasePlugin finishTransaction:result:]',
      requestMap,
    );
  }

  /// Restore previously purchased transactions.
  ///
  /// Use this to load previously purchased content on a new device.
  ///
  /// This call triggers purchase updates on the set
  /// [SKTransactionObserverWrapper] for previously made transactions. This will
  /// invoke [SKTransactionObserverWrapper.restoreCompletedTransactions],
  /// [SKTransactionObserverWrapper.paymentQueueRestoreCompletedTransactionsFinished],
  /// and [SKTransactionObserverWrapper.updatedTransaction]. These restored
  /// transactions need to be marked complete with [finishTransaction] once the
  /// content is delivered, like any other transaction.
  ///
  /// The `applicationUserName` should match the original
  /// [SKPaymentWrapper.applicationUsername] used in [addPayment].
  /// If no `applicationUserName` was used, `applicationUserName` should be null.
  ///
  /// This method either triggers [`-[SKPayment
  /// restoreCompletedTransactions]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/1506123-restorecompletedtransactions?language=objc)
  /// or [`-[SKPayment restoreCompletedTransactionsWithApplicationUsername:]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/1505992-restorecompletedtransactionswith?language=objc)
  /// depending on whether the `applicationUserName` is set.
  Future<void> restoreTransactions({String? applicationUserName}) async {
    await channel.invokeMethod<void>(
        '-[InAppPurchasePlugin restoreTransactions:result:]',
        applicationUserName);
  }

  /// Present Code Redemption Sheet
  ///
  /// Use this to allow Users to enter and redeem Codes
  ///
  /// This method triggers [`-[SKPayment
  /// presentCodeRedemptionSheet]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/3566726-presentcoderedemptionsheet?language=objc)
  Future<void> presentCodeRedemptionSheet() async {
    await channel.invokeMethod<void>(
        '-[InAppPurchasePlugin presentCodeRedemptionSheet:result:]');
  }

  /// Shows the price consent sheet if the user has not yet responded to a
  /// subscription price change.
  ///
  /// Use this function when you have registered a [SKPaymentQueueDelegateWrapper]
  /// (using the [setDelegate] method) and returned `false` when the
  /// `SKPaymentQueueDelegateWrapper.shouldShowPriceConsent()` method was called.
  ///
  /// See documentation of StoreKit's [`-[SKPaymentQueue showPriceConsentIfNeeded]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/3521327-showpriceconsentifneeded?language=objc).
  Future<void> showPriceConsentIfNeeded() async {
    await channel
        .invokeMethod<void>('-[SKPaymentQueue showPriceConsentIfNeeded]');
  }

  /// Triage a method channel call from the platform and triggers the correct observer method.
  ///
  /// This method is public for testing purposes only and should not be used
  /// outside this class.
  @visibleForTesting
  Future<dynamic> handleObserverCallbacks(MethodCall call) async {
    assert(_observer != null,
        '[in_app_purchase]: (Fatal)The observer has not been set but we received a purchase transaction notification. Please ensure the observer has been set using `setTransactionObserver`. Make sure the observer is added right at the App Launch.');
    final SKTransactionObserverWrapper observer = _observer!;
    switch (call.method) {
      case 'updatedTransactions':
        {
          final List<SKPaymentTransactionWrapper> transactions =
              _getTransactionList(call.arguments);
          return Future<void>(() {
            observer.updatedTransactions(transactions: transactions);
          });
        }
      case 'removedTransactions':
        {
          final List<SKPaymentTransactionWrapper> transactions =
              _getTransactionList(call.arguments);
          return Future<void>(() {
            observer.removedTransactions(transactions: transactions);
          });
        }
      case 'restoreCompletedTransactionsFailed':
        {
          SKError error =
              SKError.fromJson(Map<String, dynamic>.from(call.arguments));
          return Future<void>(() {
            observer.restoreCompletedTransactionsFailed(error: error);
          });
        }
      case 'paymentQueueRestoreCompletedTransactionsFinished':
        {
          return Future<void>(() {
            observer.paymentQueueRestoreCompletedTransactionsFinished();
          });
        }
      case 'shouldAddStorePayment':
        {
          SKPaymentWrapper payment =
              SKPaymentWrapper.fromJson(call.arguments['payment']);
          SKProductWrapper product =
              SKProductWrapper.fromJson(call.arguments['product']);
          return Future<void>(() {
            if (observer.shouldAddStorePayment(
                    payment: payment, product: product) ==
                true) {
              SKPaymentQueueWrapper().addPayment(payment);
            }
          });
        }
      default:
        break;
    }
    throw PlatformException(
        code: 'no_such_callback',
        message: 'Did not recognize the observer callback ${call.method}.');
  }

  // Get transaction wrapper object list from arguments.
  List<SKPaymentTransactionWrapper> _getTransactionList(
      List<dynamic> transactionsData) {
    return transactionsData.map<SKPaymentTransactionWrapper>((dynamic map) {
      return SKPaymentTransactionWrapper.fromJson(
          Map.castFrom<dynamic, dynamic, String, dynamic>(map));
    }).toList();
  }

  /// Triage a method channel call from the platform and triggers the correct
  /// payment queue delegate method.
  ///
  /// This method is public for testing purposes only and should not be used
  /// outside this class.
  @visibleForTesting
  Future<dynamic> handlePaymentQueueDelegateCallbacks(MethodCall call) async {
    assert(_paymentQueueDelegate != null,
        '[in_app_purchase]: (Fatal)The payment queue delegate has not been set but we received a payment queue notification. Please ensure the payment queue has been set using `setDelegate`.');

    final SKPaymentQueueDelegateWrapper delegate = _paymentQueueDelegate!;
    switch (call.method) {
      case 'shouldContinueTransaction':
        final SKPaymentTransactionWrapper transaction =
            SKPaymentTransactionWrapper.fromJson(call.arguments['transaction']);
        final SKStorefrontWrapper storefront =
            SKStorefrontWrapper.fromJson(call.arguments['storefront']);
        return delegate.shouldContinueTransaction(transaction, storefront);
      case 'shouldShowPriceConsent':
        return delegate.shouldShowPriceConsent();
      default:
        break;
    }
    throw PlatformException(
        code: 'no_such_callback',
        message:
            'Did not recognize the payment queue delegate callback ${call.method}.');
  }
}

/// Dart wrapper around StoreKit's
/// [NSError](https://developer.apple.com/documentation/foundation/nserror?language=objc).
@immutable
@JsonSerializable()
class SKError {
  /// Creates a new [SKError] object with the provided information.
  const SKError(
      {required this.code, required this.domain, required this.userInfo});

  /// Constructs an instance of this from a key-value map of data.
  ///
  /// The map needs to have named string keys with values matching the names and
  /// types of all of the members on this class. The `map` parameter must not be
  /// null.
  factory SKError.fromJson(Map<String, dynamic> map) {
    return _$SKErrorFromJson(map);
  }

  /// Error [code](https://developer.apple.com/documentation/foundation/1448136-nserror_codes)
  /// as defined in the Cocoa Framework.
  @JsonKey(defaultValue: 0)
  final int code;

  /// Error
  /// [domain](https://developer.apple.com/documentation/foundation/nscocoaerrordomain?language=objc)
  /// as defined in the Cocoa Framework.
  @JsonKey(defaultValue: '')
  final String domain;

  /// A map that contains more detailed information about the error.
  ///
  /// Any key of the map must be a valid [NSErrorUserInfoKey](https://developer.apple.com/documentation/foundation/nserroruserinfokey?language=objc).
  @JsonKey(defaultValue: <String, dynamic>{})
  final Map<String, dynamic> userInfo;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKError typedOther = other as SKError;
    return typedOther.code == code &&
        typedOther.domain == domain &&
        DeepCollectionEquality.unordered()
            .equals(typedOther.userInfo, userInfo);
  }

  @override
  int get hashCode => hashValues(
        code,
        domain,
        userInfo,
      );
}

/// Dart wrapper around StoreKit's
/// [SKPayment](https://developer.apple.com/documentation/storekit/skpayment?language=objc).
///
/// Used as the parameter to initiate a payment. In general, a developer should
/// not need to create the payment object explicitly; instead, use
/// [SKPaymentQueueWrapper.addPayment] directly with a product identifier to
/// initiate a payment.
@immutable
@JsonSerializable(createToJson: true)
class SKPaymentWrapper {
  /// Creates a new [SKPaymentWrapper] with the provided information.
  const SKPaymentWrapper(
      {required this.productIdentifier,
      this.applicationUsername,
      this.requestData,
      this.quantity = 1,
      this.simulatesAskToBuyInSandbox = false});

  /// Constructs an instance of this from a key value map of data.
  ///
  /// The map needs to have named string keys with values matching the names and
  /// types of all of the members on this class. The `map` parameter must not be
  /// null.
  factory SKPaymentWrapper.fromJson(Map<String, dynamic> map) {
    assert(map != null);
    return _$SKPaymentWrapperFromJson(map);
  }

  /// Creates a Map object describes the payment object.
  Map<String, dynamic> toMap() {
    return <String, dynamic>{
      'productIdentifier': productIdentifier,
      'applicationUsername': applicationUsername,
      'requestData': requestData,
      'quantity': quantity,
      'simulatesAskToBuyInSandbox': simulatesAskToBuyInSandbox
    };
  }

  /// The id for the product that the payment is for.
  @JsonKey(defaultValue: '')
  final String productIdentifier;

  /// An opaque id for the user's account.
  ///
  /// Used to help the store detect irregular activity. See
  /// [applicationUsername](https://developer.apple.com/documentation/storekit/skpayment/1506116-applicationusername?language=objc)
  /// for more details. For example, you can use a one-way hash of the user’s
  /// account name on your server. Don’t use the Apple ID for your developer
  /// account, the user’s Apple ID, or the user’s plaintext account name on
  /// your server.
  final String? applicationUsername;

  /// Reserved for future use.
  ///
  /// The value must be null before sending the payment. If the value is not
  /// null, the payment will be rejected.
  ///
  // The iOS Platform provided this property but it is reserved for future use.
  // We also provide this property to match the iOS platform. Converted to
  // String from NSData from ios platform using UTF8Encoding. The / default is
  // null.
  final String? requestData;

  /// The amount of the product this payment is for.
  ///
  /// The default is 1. The minimum is 1. The maximum is 10.
  ///
  /// If the object is invalid, the value could be 0.
  @JsonKey(defaultValue: 0)
  final int quantity;

  /// Produces an "ask to buy" flow in the sandbox.
  ///
  /// Setting it to `true` will cause a transaction to be in the state [SKPaymentTransactionStateWrapper.deferred],
  /// which produce an "ask to buy" prompt that interrupts the the payment flow.
  ///
  /// Default is `false`.
  ///
  /// See https://developer.apple.com/in-app-purchase/ for a guide on Sandbox
  /// testing.
  final bool simulatesAskToBuyInSandbox;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKPaymentWrapper typedOther = other as SKPaymentWrapper;
    return typedOther.productIdentifier == productIdentifier &&
        typedOther.applicationUsername == applicationUsername &&
        typedOther.quantity == quantity &&
        typedOther.simulatesAskToBuyInSandbox == simulatesAskToBuyInSandbox &&
        typedOther.requestData == requestData;
  }

  @override
  int get hashCode => hashValues(productIdentifier, applicationUsername,
      quantity, simulatesAskToBuyInSandbox, requestData);

  @override
  String toString() => _$SKPaymentWrapperToJson(this).toString();
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_ios/store_kit_wrappers.dart';

/// A wrapper around
/// [`SKPaymentQueueDelegate`](https://developer.apple.com/documentation/storekit/skpaymentqueuedelegate?language=objc).
///
/// The payment queue delegate can be implementated to provide information
/// needed to complete transactions.
///
/// The [SKPaymentQueueDelegateWrapper] is only available on iOS 13 and higher.
/// Using the delegate on older iOS version will be ignored.
abstract class SKPaymentQueueDelegateWrapper {
  /// Called by the system to check whether the transaction should continue if
  /// the device's App Store storefront has changed during a transaction.
  ///
  /// - Return `true` if the transaction should continue within the updated
  /// storefront (default behaviour).
  /// - Return `false` if the transaction should be cancelled. In this case the
  /// transaction will fail with the error [SKErrorStoreProductNotAvailable](https://developer.apple.com/documentation/storekit/skerrorcode/skerrorstoreproductnotavailable?language=objc).
  ///
  /// See the documentation in StoreKit's [`[-SKPaymentQueueDelegate shouldContinueTransaction]`](https://developer.apple.com/documentation/storekit/skpaymentqueuedelegate/3242935-paymentqueue?language=objc).
  bool shouldContinueTransaction(
    SKPaymentTransactionWrapper transaction,
    SKStorefrontWrapper storefront,
  ) =>
      true;

  /// Called by the system to check whether to immediately show the price
  /// consent form.
  ///
  /// The default return value is `true`. This will inform the system to display
  /// the price consent sheet when the subscription price has been changed in
  /// App Store Connect and the subscriber has not yet taken action. See the
  /// documentation in StoreKit's [`[-SKPaymentQueueDelegate shouldShowPriceConsent:]`](https://developer.apple.com/documentation/storekit/skpaymentqueuedelegate/3521328-paymentqueueshouldshowpriceconse?language=objc).
  bool shouldShowPriceConsent() => true;
}
```

```dart
// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sk_product_wrapper.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SkProductResponseWrapper _$SkProductResponseWrapperFromJson(Map json) =>
    SkProductResponseWrapper(
      products: (json['products'] as List<dynamic>?)
              ?.map((e) => SKProductWrapper.fromJson(
                  Map<String, dynamic>.from(e as Map)))
              .toList() ??
          [],
      invalidProductIdentifiers:
          (json['invalidProductIdentifiers'] as List<dynamic>?)
                  ?.map((e) => e as String)
                  .toList() ??
              [],
    );

SKProductSubscriptionPeriodWrapper _$SKProductSubscriptionPeriodWrapperFromJson(
        Map json) =>
    SKProductSubscriptionPeriodWrapper(
      numberOfUnits: json['numberOfUnits'] as int? ?? 0,
      unit: const SKSubscriptionPeriodUnitConverter()
          .fromJson(json['unit'] as int?),
    );

SKProductDiscountWrapper _$SKProductDiscountWrapperFromJson(Map json) =>
    SKProductDiscountWrapper(
      price: json['price'] as String? ?? '',
      priceLocale:
          SKPriceLocaleWrapper.fromJson((json['priceLocale'] as Map?)?.map(
        (k, e) => MapEntry(k as String, e),
      )),
      numberOfPeriods: json['numberOfPeriods'] as int? ?? 0,
      paymentMode: const SKProductDiscountPaymentModeConverter()
          .fromJson(json['paymentMode'] as int?),
      subscriptionPeriod: SKProductSubscriptionPeriodWrapper.fromJson(
          (json['subscriptionPeriod'] as Map?)?.map(
        (k, e) => MapEntry(k as String, e),
      )),
    );

SKProductWrapper _$SKProductWrapperFromJson(Map json) => SKProductWrapper(
      productIdentifier: json['productIdentifier'] as String? ?? '',
      localizedTitle: json['localizedTitle'] as String? ?? '',
      localizedDescription: json['localizedDescription'] as String? ?? '',
      priceLocale:
          SKPriceLocaleWrapper.fromJson((json['priceLocale'] as Map?)?.map(
        (k, e) => MapEntry(k as String, e),
      )),
      subscriptionGroupIdentifier:
          json['subscriptionGroupIdentifier'] as String?,
      price: json['price'] as String? ?? '',
      subscriptionPeriod: json['subscriptionPeriod'] == null
          ? null
          : SKProductSubscriptionPeriodWrapper.fromJson(
              (json['subscriptionPeriod'] as Map?)?.map(
              (k, e) => MapEntry(k as String, e),
            )),
      introductoryPrice: json['introductoryPrice'] == null
          ? null
          : SKProductDiscountWrapper.fromJson(
              Map<String, dynamic>.from(json['introductoryPrice'] as Map)),
    );

SKPriceLocaleWrapper _$SKPriceLocaleWrapperFromJson(Map json) =>
    SKPriceLocaleWrapper(
      currencySymbol: json['currencySymbol'] as String? ?? '',
      currencyCode: json['currencyCode'] as String? ?? '',
      countryCode: json['countryCode'] as String? ?? '',
    );
```

```dart
// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sk_storefront_wrapper.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SKStorefrontWrapper _$SKStorefrontWrapperFromJson(Map json) =>
    SKStorefrontWrapper(
      countryCode: json['countryCode'] as String,
      identifier: json['identifier'] as String,
    );

Map<String, dynamic> _$SKStorefrontWrapperToJson(
        SKStorefrontWrapper instance) =>
    <String, dynamic>{
      'countryCode': instance.countryCode,
      'identifier': instance.identifier,
    };
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:ui' show hashValues;
import 'package:collection/collection.dart';
import 'package:json_annotation/json_annotation.dart';
import 'enum_converters.dart';

// WARNING: Changes to `@JsonSerializable` classes need to be reflected in the
// below generated file. Run `flutter packages pub run build_runner watch` to
// rebuild and watch for further changes.
part 'sk_product_wrapper.g.dart';

/// Dart wrapper around StoreKit's [SKProductsResponse](https://developer.apple.com/documentation/storekit/skproductsresponse?language=objc).
///
/// Represents the response object returned by [SKRequestMaker.startProductRequest].
/// Contains information about a list of products and a list of invalid product identifiers.
@JsonSerializable()
class SkProductResponseWrapper {
  /// Creates an [SkProductResponseWrapper] with the given product details.
  SkProductResponseWrapper(
      {required this.products, required this.invalidProductIdentifiers});

  /// Constructing an instance from a map from the Objective-C layer.
  ///
  /// This method should only be used with `map` values returned by [SKRequestMaker.startProductRequest].
  factory SkProductResponseWrapper.fromJson(Map<String, dynamic> map) {
    return _$SkProductResponseWrapperFromJson(map);
  }

  /// Stores all matching successfully found products.
  ///
  /// One product in this list matches one valid product identifier passed to the [SKRequestMaker.startProductRequest].
  /// Will be empty if the [SKRequestMaker.startProductRequest] method does not pass any correct product identifier.
  @JsonKey(defaultValue: <SKProductWrapper>[])
  final List<SKProductWrapper> products;

  /// Stores product identifiers in the `productIdentifiers` from [SKRequestMaker.startProductRequest] that are not recognized by the App Store.
  ///
  /// The App Store will not recognize a product identifier unless certain criteria are met. A detailed list of the criteria can be
  /// found here https://developer.apple.com/documentation/storekit/skproductsresponse/1505985-invalidproductidentifiers?language=objc.
  /// Will be empty if all the product identifiers are valid.
  @JsonKey(defaultValue: <String>[])
  final List<String> invalidProductIdentifiers;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SkProductResponseWrapper typedOther =
        other as SkProductResponseWrapper;
    return DeepCollectionEquality().equals(typedOther.products, products) &&
        DeepCollectionEquality().equals(
            typedOther.invalidProductIdentifiers, invalidProductIdentifiers);
  }

  @override
  int get hashCode => hashValues(this.products, this.invalidProductIdentifiers);
}

/// Dart wrapper around StoreKit's [SKProductPeriodUnit](https://developer.apple.com/documentation/storekit/skproductperiodunit?language=objc).
///
/// Used as a property in the [SKProductSubscriptionPeriodWrapper]. Minimum is a day and maximum is a year.
// The values of the enum options are matching the [SKProductPeriodUnit]'s values. Should there be an update or addition
// in the [SKProductPeriodUnit], this need to be updated to match.
enum SKSubscriptionPeriodUnit {
  /// An interval lasting one day.
  @JsonValue(0)
  day,

  /// An interval lasting one month.
  @JsonValue(1)

  /// An interval lasting one week.
  week,
  @JsonValue(2)

  /// An interval lasting one month.
  month,

  /// An interval lasting one year.
  @JsonValue(3)
  year,
}

/// Dart wrapper around StoreKit's [SKProductSubscriptionPeriod](https://developer.apple.com/documentation/storekit/skproductsubscriptionperiod?language=objc).
///
/// A period is defined by a [numberOfUnits] and a [unit], e.g for a 3 months period [numberOfUnits] is 3 and [unit] is a month.
/// It is used as a property in [SKProductDiscountWrapper] and [SKProductWrapper].
@JsonSerializable()
class SKProductSubscriptionPeriodWrapper {
  /// Creates an [SKProductSubscriptionPeriodWrapper] for a `numberOfUnits`x`unit` period.
  SKProductSubscriptionPeriodWrapper(
      {required this.numberOfUnits, required this.unit});

  /// Constructing an instance from a map from the Objective-C layer.
  ///
  /// This method should only be used with `map` values returned by [SKProductDiscountWrapper.fromJson] or [SKProductWrapper.fromJson].
  factory SKProductSubscriptionPeriodWrapper.fromJson(
      Map<String, dynamic>? map) {
    if (map == null) {
      return SKProductSubscriptionPeriodWrapper(
          numberOfUnits: 0, unit: SKSubscriptionPeriodUnit.day);
    }
    return _$SKProductSubscriptionPeriodWrapperFromJson(map);
  }

  /// The number of [unit] units in this period.
  ///
  /// Must be greater than 0 if the object is valid.
  @JsonKey(defaultValue: 0)
  final int numberOfUnits;

  /// The time unit used to specify the length of this period.
  @SKSubscriptionPeriodUnitConverter()
  final SKSubscriptionPeriodUnit unit;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKProductSubscriptionPeriodWrapper typedOther =
        other as SKProductSubscriptionPeriodWrapper;
    return typedOther.numberOfUnits == numberOfUnits && typedOther.unit == unit;
  }

  @override
  int get hashCode => hashValues(this.numberOfUnits, this.unit);
}

/// Dart wrapper around StoreKit's [SKProductDiscountPaymentMode](https://developer.apple.com/documentation/storekit/skproductdiscountpaymentmode?language=objc).
///
/// This is used as a property in the [SKProductDiscountWrapper].
// The values of the enum options are matching the [SKProductDiscountPaymentMode]'s values. Should there be an update or addition
// in the [SKProductDiscountPaymentMode], this need to be updated to match.
enum SKProductDiscountPaymentMode {
  /// Allows user to pay the discounted price at each payment period.
  @JsonValue(0)
  payAsYouGo,

  /// Allows user to pay the discounted price upfront and receive the product for the rest of time that was paid for.
  @JsonValue(1)
  payUpFront,

  /// User pays nothing during the discounted period.
  @JsonValue(2)
  freeTrail,

  /// Unspecified mode.
  @JsonValue(-1)
  unspecified,
}

/// Dart wrapper around StoreKit's [SKProductDiscount](https://developer.apple.com/documentation/storekit/skproductdiscount?language=objc).
///
/// It is used as a property in [SKProductWrapper].
@JsonSerializable()
class SKProductDiscountWrapper {
  /// Creates an [SKProductDiscountWrapper] with the given discount details.
  SKProductDiscountWrapper(
      {required this.price,
      required this.priceLocale,
      required this.numberOfPeriods,
      required this.paymentMode,
      required this.subscriptionPeriod});

  /// Constructing an instance from a map from the Objective-C layer.
  ///
  /// This method should only be used with `map` values returned by [SKProductWrapper.fromJson].
  factory SKProductDiscountWrapper.fromJson(Map<String, dynamic> map) {
    return _$SKProductDiscountWrapperFromJson(map);
  }

  /// The discounted price, in the currency that is defined in [priceLocale].
  @JsonKey(defaultValue: '')
  final String price;

  /// Includes locale information about the price, e.g. `$` as the currency symbol for US locale.
  final SKPriceLocaleWrapper priceLocale;

  /// The object represent the discount period length.
  ///
  /// The value must be >= 0 if the object is valid.
  @JsonKey(defaultValue: 0)
  final int numberOfPeriods;

  /// The object indicates how the discount price is charged.
  @SKProductDiscountPaymentModeConverter()
  final SKProductDiscountPaymentMode paymentMode;

  /// The object represents the duration of single subscription period for the discount.
  ///
  /// The [subscriptionPeriod] of the discount is independent of the product's [subscriptionPeriod],
  /// and their units and duration do not have to be matched.
  final SKProductSubscriptionPeriodWrapper subscriptionPeriod;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKProductDiscountWrapper typedOther =
        other as SKProductDiscountWrapper;
    return typedOther.price == price &&
        typedOther.priceLocale == priceLocale &&
        typedOther.numberOfPeriods == numberOfPeriods &&
        typedOther.paymentMode == paymentMode &&
        typedOther.subscriptionPeriod == subscriptionPeriod;
  }

  @override
  int get hashCode => hashValues(this.price, this.priceLocale,
      this.numberOfPeriods, this.paymentMode, this.subscriptionPeriod);
}

/// Dart wrapper around StoreKit's [SKProduct](https://developer.apple.com/documentation/storekit/skproduct?language=objc).
///
/// A list of [SKProductWrapper] is returned in the [SKRequestMaker.startProductRequest] method, and
/// should be stored for use when making a payment.
@JsonSerializable()
class SKProductWrapper {
  /// Creates an [SKProductWrapper] with the given product details.
  SKProductWrapper({
    required this.productIdentifier,
    required this.localizedTitle,
    required this.localizedDescription,
    required this.priceLocale,
    this.subscriptionGroupIdentifier,
    required this.price,
    this.subscriptionPeriod,
    this.introductoryPrice,
  });

  /// Constructing an instance from a map from the Objective-C layer.
  ///
  /// This method should only be used with `map` values returned by [SkProductResponseWrapper.fromJson].
  factory SKProductWrapper.fromJson(Map<String, dynamic> map) {
    return _$SKProductWrapperFromJson(map);
  }

  /// The unique identifier of the product.
  @JsonKey(defaultValue: '')
  final String productIdentifier;

  /// The localizedTitle of the product.
  ///
  /// It is localized based on the current locale.
  @JsonKey(defaultValue: '')
  final String localizedTitle;

  /// The localized description of the product.
  ///
  /// It is localized based on the current locale.
  @JsonKey(defaultValue: '')
  final String localizedDescription;

  /// Includes locale information about the price, e.g. `$` as the currency symbol for US locale.
  final SKPriceLocaleWrapper priceLocale;

  /// The subscription group identifier.
  ///
  /// If the product is not a subscription, the value is `null`.
  ///
  /// A subscription group is a collection of subscription products.
  /// Check [SubscriptionGroup](https://developer.apple.com/app-store/subscriptions/) for more details about subscription group.
  final String? subscriptionGroupIdentifier;

  /// The price of the product, in the currency that is defined in [priceLocale].
  @JsonKey(defaultValue: '')
  final String price;

  /// The object represents the subscription period of the product.
  ///
  /// Can be [null] is the product is not a subscription.
  final SKProductSubscriptionPeriodWrapper? subscriptionPeriod;

  /// The object represents the duration of single subscription period.
  ///
  /// This is only available if you set up the introductory price in the App Store Connect, otherwise the value is `null`.
  /// Programmer is also responsible to determine if the user is eligible to receive it. See https://developer.apple.com/documentation/storekit/in-app_purchase/offering_introductory_pricing_in_your_app?language=objc
  /// for more details.
  /// The [subscriptionPeriod] of the discount is independent of the product's [subscriptionPeriod],
  /// and their units and duration do not have to be matched.
  final SKProductDiscountWrapper? introductoryPrice;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKProductWrapper typedOther = other as SKProductWrapper;
    return typedOther.productIdentifier == productIdentifier &&
        typedOther.localizedTitle == localizedTitle &&
        typedOther.localizedDescription == localizedDescription &&
        typedOther.priceLocale == priceLocale &&
        typedOther.subscriptionGroupIdentifier == subscriptionGroupIdentifier &&
        typedOther.price == price &&
        typedOther.subscriptionPeriod == subscriptionPeriod &&
        typedOther.introductoryPrice == introductoryPrice;
  }

  @override
  int get hashCode => hashValues(
      this.productIdentifier,
      this.localizedTitle,
      this.localizedDescription,
      this.priceLocale,
      this.subscriptionGroupIdentifier,
      this.price,
      this.subscriptionPeriod,
      this.introductoryPrice);
}

/// Object that indicates the locale of the price
///
/// It is a thin wrapper of [NSLocale](https://developer.apple.com/documentation/foundation/nslocale?language=objc).
// TODO(cyanglaz): NSLocale is a complex object, want to see the actual need of getting this expanded.
//                 Matching android to only get the currencySymbol for now.
//                 https://github.com/flutter/flutter/issues/26610
@JsonSerializable()
class SKPriceLocaleWrapper {
  /// Creates a new price locale for `currencySymbol` and `currencyCode`.
  SKPriceLocaleWrapper({
    required this.currencySymbol,
    required this.currencyCode,
    required this.countryCode,
  });

  /// Constructing an instance from a map from the Objective-C layer.
  ///
  /// This method should only be used with `map` values returned by [SKProductWrapper.fromJson] and [SKProductDiscountWrapper.fromJson].
  factory SKPriceLocaleWrapper.fromJson(Map<String, dynamic>? map) {
    if (map == null) {
      return SKPriceLocaleWrapper(
          currencyCode: '', currencySymbol: '', countryCode: '');
    }
    return _$SKPriceLocaleWrapperFromJson(map);
  }

  ///The currency symbol for the locale, e.g. $ for US locale.
  @JsonKey(defaultValue: '')
  final String currencySymbol;

  ///The currency code for the locale, e.g. USD for US locale.
  @JsonKey(defaultValue: '')
  final String currencyCode;

  ///The country code for the locale, e.g. US for US locale.
  @JsonKey(defaultValue: '')
  final String countryCode;

  @override
  bool operator ==(Object other) {
    if (identical(other, this)) {
      return true;
    }
    if (other.runtimeType != runtimeType) {
      return false;
    }
    final SKPriceLocaleWrapper typedOther = other as SKPriceLocaleWrapper;
    return typedOther.currencySymbol == currencySymbol &&
        typedOther.currencyCode == currencyCode;
  }

  @override
  int get hashCode => hashValues(this.currencySymbol, this.currencyCode);
}
```

```dart
// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'enum_converters.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_SerializedEnums _$SerializedEnumsFromJson(Map json) => _SerializedEnums()
  ..response =
      $enumDecode(_$SKPaymentTransactionStateWrapperEnumMap, json['response'])
  ..unit = $enumDecode(_$SKSubscriptionPeriodUnitEnumMap, json['unit'])
  ..discountPaymentMode = $enumDecode(
      _$SKProductDiscountPaymentModeEnumMap, json['discountPaymentMode']);

const _$SKPaymentTransactionStateWrapperEnumMap = {
  SKPaymentTransactionStateWrapper.purchasing: 0,
  SKPaymentTransactionStateWrapper.purchased: 1,
  SKPaymentTransactionStateWrapper.failed: 2,
  SKPaymentTransactionStateWrapper.restored: 3,
  SKPaymentTransactionStateWrapper.deferred: 4,
  SKPaymentTransactionStateWrapper.unspecified: -1,
};

const _$SKSubscriptionPeriodUnitEnumMap = {
  SKSubscriptionPeriodUnit.day: 0,
  SKSubscriptionPeriodUnit.week: 1,
  SKSubscriptionPeriodUnit.month: 2,
  SKSubscriptionPeriodUnit.year: 3,
};

const _$SKProductDiscountPaymentModeEnumMap = {
  SKProductDiscountPaymentMode.payAsYouGo: 0,
  SKProductDiscountPaymentMode.payUpFront: 1,
  SKProductDiscountPaymentMode.freeTrail: 2,
  SKProductDiscountPaymentMode.unspecified: -1,
};
```

```dart
// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sk_payment_queue_wrapper.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SKError _$SKErrorFromJson(Map json) => SKError(
      code: json['code'] as int? ?? 0,
      domain: json['domain'] as String? ?? '',
      userInfo: (json['userInfo'] as Map?)?.map(
            (k, e) => MapEntry(k as String, e),
          ) ??
          {},
    );

SKPaymentWrapper _$SKPaymentWrapperFromJson(Map json) => SKPaymentWrapper(
      productIdentifier: json['productIdentifier'] as String? ?? '',
      applicationUsername: json['applicationUsername'] as String?,
      requestData: json['requestData'] as String?,
      quantity: json['quantity'] as int? ?? 0,
      simulatesAskToBuyInSandbox:
          json['simulatesAskToBuyInSandbox'] as bool? ?? false,
    );

Map<String, dynamic> _$SKPaymentWrapperToJson(SKPaymentWrapper instance) =>
    <String, dynamic>{
      'productIdentifier': instance.productIdentifier,
      'applicationUsername': instance.applicationUsername,
      'requestData': instance.requestData,
      'quantity': instance.quantity,
      'simulatesAskToBuyInSandbox': instance.simulatesAskToBuyInSandbox,
    };
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:in_app_purchase_ios/src/in_app_purchase_ios_platform_addition.dart';
import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import '../in_app_purchase_ios.dart';
import '../store_kit_wrappers.dart';

/// [IAPError.code] code for failed purchases.
const String kPurchaseErrorCode = 'purchase_error';

/// Indicates store front is Apple AppStore.
const String kIAPSource = 'app_store';

/// An [InAppPurchasePlatform] that wraps StoreKit.
///
/// This translates various `StoreKit` calls and responses into the
/// generic plugin API.
class InAppPurchaseIosPlatform extends InAppPurchasePlatform {
  static late SKPaymentQueueWrapper _skPaymentQueueWrapper;
  static late _TransactionObserver _observer;

  /// Creates an [InAppPurchaseIosPlatform] object.
  ///
  /// This constructor should only be used for testing, for any other purpose
  /// get the connection from the [instance] getter.
  @visibleForTesting
  InAppPurchaseIosPlatform();

  Stream<List<PurchaseDetails>> get purchaseStream =>
      _observer.purchaseUpdatedController.stream;

  /// Callback handler for transaction status changes.
  @visibleForTesting
  static SKTransactionObserverWrapper get observer => _observer;

  /// Registers this class as the default instance of [InAppPurchasePlatform].
  static void registerPlatform() {
    // Register the [InAppPurchaseIosPlatformAddition] containing iOS
    // platform-specific functionality.
    InAppPurchasePlatformAddition.instance = InAppPurchaseIosPlatformAddition();

    // Register the platform-specific implementation of the idiomatic
    // InAppPurchase API.
    InAppPurchasePlatform.instance = InAppPurchaseIosPlatform();

    _skPaymentQueueWrapper = SKPaymentQueueWrapper();

    // Create a purchaseUpdatedController and notify the native side when to
    // start of stop sending updates.
    StreamController<List<PurchaseDetails>> updateController =
        StreamController.broadcast(
      onListen: () => _skPaymentQueueWrapper.startObservingTransactionQueue(),
      onCancel: () => _skPaymentQueueWrapper.stopObservingTransactionQueue(),
    );
    _observer = _TransactionObserver(updateController);
    _skPaymentQueueWrapper.setTransactionObserver(observer);
  }

  @override
  Future<bool> isAvailable() => SKPaymentQueueWrapper.canMakePayments();

  @override
  Future<bool> buyNonConsumable({required PurchaseParam purchaseParam}) async {
    await _skPaymentQueueWrapper.addPayment(SKPaymentWrapper(
        productIdentifier: purchaseParam.productDetails.id,
        quantity: 1,
        applicationUsername: purchaseParam.applicationUserName,
        simulatesAskToBuyInSandbox: (purchaseParam is AppStorePurchaseParam)
            ? purchaseParam.simulatesAskToBuyInSandbox
            : false,
        requestData: null));

    return true; // There's no error feedback from iOS here to return.
  }

  @override
  Future<bool> buyConsumable(
      {required PurchaseParam purchaseParam, bool autoConsume = true}) {
    assert(autoConsume == true, 'On iOS, we should always auto consume');
    return buyNonConsumable(purchaseParam: purchaseParam);
  }

  @override
  Future<void> completePurchase(PurchaseDetails purchase) {
    assert(
      purchase is AppStorePurchaseDetails,
      'On iOS, the `purchase` should always be of type `AppStorePurchaseDetails`.',
    );

    return _skPaymentQueueWrapper.finishTransaction(
      (purchase as AppStorePurchaseDetails).skPaymentTransaction,
    );
  }

  @override
  Future<void> restorePurchases({String? applicationUserName}) async {
    return _observer
        .restoreTransactions(
            queue: _skPaymentQueueWrapper,
            applicationUserName: applicationUserName)
        .whenComplete(() => _observer.cleanUpRestoredTransactions());
  }

  /// Query the product detail list.
  ///
  /// This method only returns [ProductDetailsResponse].
  /// To get detailed Store Kit product list, use [SkProductResponseWrapper.startProductRequest]
  /// to get the [SKProductResponseWrapper].
  @override
  Future<ProductDetailsResponse> queryProductDetails(
      Set<String> identifiers) async {
    final SKRequestMaker requestMaker = SKRequestMaker();
    SkProductResponseWrapper response;
    PlatformException? exception;
    try {
      response = await requestMaker.startProductRequest(identifiers.toList());
    } on PlatformException catch (e) {
      exception = e;
      response = SkProductResponseWrapper(
          products: [], invalidProductIdentifiers: identifiers.toList());
    }
    List<AppStoreProductDetails> productDetails = [];
    if (response.products != null) {
      productDetails = response.products
          .map((SKProductWrapper productWrapper) =>
              AppStoreProductDetails.fromSKProduct(productWrapper))
          .toList();
    }
    List<String> invalidIdentifiers = response.invalidProductIdentifiers;
    if (productDetails.isEmpty) {
      invalidIdentifiers = identifiers.toList();
    }
    ProductDetailsResponse productDetailsResponse = ProductDetailsResponse(
      productDetails: productDetails,
      notFoundIDs: invalidIdentifiers,
      error: exception == null
          ? null
          : IAPError(
              source: kIAPSource,
              code: exception.code,
              message: exception.message ?? '',
              details: exception.details),
    );
    return productDetailsResponse;
  }
}

class _TransactionObserver implements SKTransactionObserverWrapper {
  final StreamController<List<PurchaseDetails>> purchaseUpdatedController;

  Completer? _restoreCompleter;
  late String _receiptData;

  _TransactionObserver(this.purchaseUpdatedController);

  Future<void> restoreTransactions({
    required SKPaymentQueueWrapper queue,
    String? applicationUserName,
  }) {
    _restoreCompleter = Completer();
    queue.restoreTransactions(applicationUserName: applicationUserName);
    return _restoreCompleter!.future;
  }

  void cleanUpRestoredTransactions() {
    _restoreCompleter = null;
  }

  void updatedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions}) async {
    String receiptData = await getReceiptData();
    List<PurchaseDetails> purchases = transactions
        .map((SKPaymentTransactionWrapper transaction) =>
            AppStorePurchaseDetails.fromSKTransaction(transaction, receiptData))
        .toList();

    purchaseUpdatedController.add(purchases);
  }

  void removedTransactions(
      {required List<SKPaymentTransactionWrapper> transactions}) {}

  /// Triggered when there is an error while restoring transactions.
  void restoreCompletedTransactionsFailed({required SKError error}) {
    _restoreCompleter!.completeError(error);
  }

  void paymentQueueRestoreCompletedTransactionsFinished() {
    _restoreCompleter!.complete();
  }

  bool shouldAddStorePayment(
      {required SKPaymentWrapper payment, required SKProductWrapper product}) {
    // In this unified API, we always return true to keep it consistent with the behavior on Google Play.
    return true;
  }

  Future<String> getReceiptData() async {
    try {
      _receiptData = await SKReceiptManager.retrieveReceiptData();
    } catch (e) {
      _receiptData = '';
    }
    return _receiptData;
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:in_app_purchase_ios/in_app_purchase_ios.dart';
import 'package:in_app_purchase_platform_interface/in_app_purchase_platform_interface.dart';

import '../store_kit_wrappers.dart';

/// Contains InApp Purchase features that are only available on iOS.
class InAppPurchaseIosPlatformAddition extends InAppPurchasePlatformAddition {
  /// Present Code Redemption Sheet.
  ///
  /// Available on devices running iOS 14 and iPadOS 14 and later.
  Future presentCodeRedemptionSheet() {
    return SKPaymentQueueWrapper().presentCodeRedemptionSheet();
  }

  /// Retry loading purchase data after an initial failure.
  ///
  /// If no results, a `null` value is returned.
  Future<PurchaseVerificationData?> refreshPurchaseVerificationData() async {
    await SKRequestMaker().startRefreshReceiptRequest();
    try {
      String receipt = await SKReceiptManager.retrieveReceiptData();
      return PurchaseVerificationData(
          localVerificationData: receipt,
          serverVerificationData: receipt,
          source: kIAPSource);
    } catch (e) {
      print(
          'Something is wrong while fetching the receipt, this normally happens when the app is '
          'running on a simulator: $e');
      return null;
    }
  }

  /// Sets an implementation of the [SKPaymentQueueDelegateWrapper].
  ///
  /// The [SKPaymentQueueDelegateWrapper] can be used to inform iOS how to
  /// finish transactions when the storefront changes or if the price consent
  /// sheet should be displayed when the price of a subscription has changed. If
  /// no delegate is registered iOS will fallback to it's default configuration.
  /// See the documentation on StoreKite's [`-[SKPaymentQueue delegate:]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/3182429-delegate?language=objc).
  ///
  /// When set to `null` the payment queue delegate will be removed and the
  /// default behaviour will apply (see [documentation](https://developer.apple.com/documentation/storekit/skpaymentqueue/3182429-delegate?language=objc)).
  Future setDelegate(SKPaymentQueueDelegateWrapper? delegate) =>
      SKPaymentQueueWrapper().setDelegate(delegate);

  /// Shows the price consent sheet if the user has not yet responded to a
  /// subscription price change.
  ///
  /// Use this function when you have registered a [SKPaymentQueueDelegateWrapper]
  /// (using the [setDelegate] method) and returned `false` when the
  /// `SKPaymentQueueDelegateWrapper.shouldShowPriceConsent()` method was called.
  ///
  /// See documentation of StoreKit's [`-[SKPaymentQueue showPriceConsentIfNeeded]`](https://developer.apple.com/documentation/storekit/skpaymentqueue/3521327-showpriceconsentifneeded?language=objc).
  Future showPriceConsentIfNeeded() =>
      SKPaymentQueueWrapper().showPriceConsentIfNeeded();
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter/services.dart';

/// Method channel for the plugin's platform<-->Dart calls.
const MethodChannel channel =
    MethodChannel('plugins.flutter.io/in_app_purchase');

/// Method channel used to deliver the payment queue delegate system calls to
/// Dart.
const MethodChannel paymentQueueDelegateChannel =
    MethodChannel('plugins.flutter.io/in_app_purchase_payment_queue_delegate');
```

```dart
// Copyright 2019, the Chromium project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

// @dart=2.9

import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:device_info_plus/device_info_plus.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  IosDeviceInfo iosInfo;
  AndroidDeviceInfo androidInfo;
  WebBrowserInfo webBrowserInfo;
  WindowsDeviceInfo windowsInfo;
  LinuxDeviceInfo linuxInfo;
  MacOsDeviceInfo macosInfo;
  BaseDeviceInfo deviceInfo;

  setUpAll(() async {
    final deviceInfoPlugin = DeviceInfoPlugin();
    if (kIsWeb) {
      webBrowserInfo = await deviceInfoPlugin.webBrowserInfo;
    } else {
      if (Platform.isIOS) {
        iosInfo = await deviceInfoPlugin.iosInfo;
      } else if (Platform.isAndroid) {
        androidInfo = await deviceInfoPlugin.androidInfo;
      } else if (Platform.isWindows) {
        windowsInfo = await deviceInfoPlugin.windowsInfo;
      } else if (Platform.isLinux) {
        linuxInfo = await deviceInfoPlugin.linuxInfo;
      } else if (Platform.isMacOS) {
        macosInfo = await deviceInfoPlugin.macOsInfo;
      }
    }

    deviceInfo = await deviceInfoPlugin.deviceInfo;
  });

  testWidgets('Can get non-null device model', (WidgetTester tester) async {
    if (kIsWeb) {
      expect(webBrowserInfo.userAgent, isNotNull);
      expect(deviceInfo, same(webBrowserInfo));
    } else {
      if (Platform.isIOS) {
        expect(iosInfo.model, isNotNull);
        expect(deviceInfo, same(iosInfo));
      } else if (Platform.isAndroid) {
        expect(androidInfo.model, isNotNull);
        expect(deviceInfo, same(androidInfo));
      } else if (Platform.isWindows) {
        expect(windowsInfo.computerName, isNotNull);
        expect(deviceInfo, same(windowsInfo));
      } else if (Platform.isLinux) {
        expect(linuxInfo.name, isNotNull);
        expect(deviceInfo, same(linuxInfo));
      } else if (Platform.isMacOS) {
        expect(macosInfo.computerName, isNotNull);
        expect(deviceInfo, same(macosInfo));
      }
    }
  });
}
```

```dart
// Copyright 2019, the Chromium project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

// @dart=2.9

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ignore_for_file: public_member_api_docs

import 'dart:async';
import 'dart:developer' as developer;
import 'dart:io';

import 'package:device_info_plus/device_info_plus.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

void main() {
  runZonedGuarded(() {
    runApp(const MyApp());
  }, (dynamic error, dynamic stack) {
    developer.log("Something went wrong!", error: error, stackTrace: stack);
  });
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  static final DeviceInfoPlugin deviceInfoPlugin = DeviceInfoPlugin();
  Map<String, dynamic> _deviceData = <String, dynamic>{};

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  Future<void> initPlatformState() async {
    var deviceData = <String, dynamic>{};

    try {
      if (kIsWeb) {
        deviceData = _readWebBrowserInfo(await deviceInfoPlugin.webBrowserInfo);
      } else {
        if (Platform.isAndroid) {
          deviceData =
              _readAndroidBuildData(await deviceInfoPlugin.androidInfo);
        } else if (Platform.isIOS) {
          deviceData = _readIosDeviceInfo(await deviceInfoPlugin.iosInfo);
        } else if (Platform.isLinux) {
          deviceData = _readLinuxDeviceInfo(await deviceInfoPlugin.linuxInfo);
        } else if (Platform.isMacOS) {
          deviceData = _readMacOsDeviceInfo(await deviceInfoPlugin.macOsInfo);
        } else if (Platform.isWindows) {
          deviceData =
              _readWindowsDeviceInfo(await deviceInfoPlugin.windowsInfo);
        }
      }
    } on PlatformException {
      deviceData = <String, dynamic>{
        'Error:': 'Failed to get platform version.'
      };
    }

    if (!mounted) return;

    setState(() {
      _deviceData = deviceData;
    });
  }

  Map<String, dynamic> _readAndroidBuildData(AndroidDeviceInfo build) {
    return <String, dynamic>{
      'version.securityPatch': build.version.securityPatch,
      'version.sdkInt': build.version.sdkInt,
      'version.release': build.version.release,
      'version.previewSdkInt': build.version.previewSdkInt,
      'version.incremental': build.version.incremental,
      'version.codename': build.version.codename,
      'version.baseOS': build.version.baseOS,
      'board': build.board,
      'bootloader': build.bootloader,
      'brand': build.brand,
      'device': build.device,
      'display': build.display,
      'fingerprint': build.fingerprint,
      'hardware': build.hardware,
      'host': build.host,
      'id': build.id,
      'manufacturer': build.manufacturer,
      'model': build.model,
      'product': build.product,
      'supported32BitAbis': build.supported32BitAbis,
      'supported64BitAbis': build.supported64BitAbis,
      'supportedAbis': build.supportedAbis,
      'tags': build.tags,
      'type': build.type,
      'isPhysicalDevice': build.isPhysicalDevice,
      'androidId': build.androidId,
      'systemFeatures': build.systemFeatures,
    };
  }

  Map<String, dynamic> _readIosDeviceInfo(IosDeviceInfo data) {
    return <String, dynamic>{
      'name': data.name,
      'systemName': data.systemName,
      'systemVersion': data.systemVersion,
      'model': data.model,
      'localizedModel': data.localizedModel,
      'identifierForVendor': data.identifierForVendor,
      'isPhysicalDevice': data.isPhysicalDevice,
      'utsname.sysname:': data.utsname.sysname,
      'utsname.nodename:': data.utsname.nodename,
      'utsname.release:': data.utsname.release,
      'utsname.version:': data.utsname.version,
      'utsname.machine:': data.utsname.machine,
    };
  }

  Map<String, dynamic> _readLinuxDeviceInfo(LinuxDeviceInfo data) {
    return <String, dynamic>{
      'name': data.name,
      'version': data.version,
      'id': data.id,
      'idLike': data.idLike,
      'versionCodename': data.versionCodename,
      'versionId': data.versionId,
      'prettyName': data.prettyName,
      'buildId': data.buildId,
      'variant': data.variant,
      'variantId': data.variantId,
      'machineId': data.machineId,
    };
  }

  Map<String, dynamic> _readWebBrowserInfo(WebBrowserInfo data) {
    return <String, dynamic>{
      'browserName': describeEnum(data.browserName),
      'appCodeName': data.appCodeName,
      'appName': data.appName,
      'appVersion': data.appVersion,
      'deviceMemory': data.deviceMemory,
      'language': data.language,
      'languages': data.languages,
      'platform': data.platform,
      'product': data.product,
      'productSub': data.productSub,
      'userAgent': data.userAgent,
      'vendor': data.vendor,
      'vendorSub': data.vendorSub,
      'hardwareConcurrency': data.hardwareConcurrency,
      'maxTouchPoints': data.maxTouchPoints,
    };
  }

  Map<String, dynamic> _readMacOsDeviceInfo(MacOsDeviceInfo data) {
    return <String, dynamic>{
      'computerName': data.computerName,
      'hostName': data.hostName,
      'arch': data.arch,
      'model': data.model,
      'kernelVersion': data.kernelVersion,
      'osRelease': data.osRelease,
      'activeCPUs': data.activeCPUs,
      'memorySize': data.memorySize,
      'cpuFrequency': data.cpuFrequency,
      'systemGUID': data.systemGUID,
    };
  }

  Map<String, dynamic> _readWindowsDeviceInfo(WindowsDeviceInfo data) {
    return <String, dynamic>{
      'numberOfCores': data.numberOfCores,
      'computerName': data.computerName,
      'systemMemoryInMegabytes': data.systemMemoryInMegabytes,
    };
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text(
            kIsWeb
                ? 'Web Browser info'
                : Platform.isAndroid
                    ? 'Android Device Info'
                    : Platform.isIOS
                        ? 'iOS Device Info'
                        : Platform.isLinux
                            ? 'Linux Device Info'
                            : Platform.isMacOS
                                ? 'MacOS Device Info'
                                : Platform.isWindows
                                    ? 'Windows Device Info'
                                    : '',
          ),
        ),
        body: ListView(
          children: _deviceData.keys.map(
            (String property) {
              return Row(
                children: <Widget>[
                  Container(
                    padding: const EdgeInsets.all(10.0),
                    child: Text(
                      property,
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  Expanded(
                      child: Container(
                    padding: const EdgeInsets.fromLTRB(0.0, 10.0, 0.0, 10.0),
                    child: Text(
                      '${_deviceData[property]}',
                      maxLines: 10,
                      overflow: TextOverflow.ellipsis,
                    ),
                  )),
                ],
              );
            },
          ).toList(),
        ),
      ),
    );
  }
}
```

```dart
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:io';

import 'package:device_info_plus_platform_interface/device_info_plus_platform_interface.dart';
import 'package:flutter/foundation.dart';
export 'package:device_info_plus_platform_interface/device_info_plus_platform_interface.dart'
    show
        AndroidBuildVersion,
        AndroidDeviceInfo,
        BaseDeviceInfo,
        IosDeviceInfo,
        IosUtsname,
        LinuxDeviceInfo,
        MacOsDeviceInfo,
        WindowsDeviceInfo,
        WebBrowserInfo,
        BrowserName;

/// Provides device and operating system information.
class DeviceInfoPlugin {
  /// No work is done when instantiating the plugin. It's safe to call this
  /// repeatedly or in performance-sensitive blocks.
  DeviceInfoPlugin();

  // This is to manually endorse the Linux plugin until automatic registration
  // of dart plugins is implemented.
  // See https://github.com/flutter/flutter/issues/52267 for more details.
  static DeviceInfoPlatform get _platform {
    return DeviceInfoPlatform.instance;
  }

  /// This information does not change from call to call. Cache it.
  AndroidDeviceInfo? _cachedAndroidDeviceInfo;

  /// Information derived from `android.os.Build`.
  ///
  /// See: https://developer.android.com/reference/android/os/Build.html
  Future<AndroidDeviceInfo> get androidInfo async =>
      _cachedAndroidDeviceInfo ??= await _platform.androidInfo();

  /// This information does not change from call to call. Cache it.
  IosDeviceInfo? _cachedIosDeviceInfo;

  /// Information derived from `UIDevice`.
  ///
  /// See: https://developer.apple.com/documentation/uikit/uidevice
  Future<IosDeviceInfo> get iosInfo async =>
      _cachedIosDeviceInfo ??= await _platform.iosInfo();

  /// This information does not change from call to call. Cache it.
  LinuxDeviceInfo? _cachedLinuxDeviceInfo;

  /// Information derived from `/etc/os-release`.
  ///
  /// See: https://www.freedesktop.org/software/systemd/man/os-release.html
  Future<LinuxDeviceInfo> get linuxInfo async =>
      _cachedLinuxDeviceInfo ??= await _platform.linuxInfo();

  /// This information does not change from call to call. Cache it.
  WebBrowserInfo? _cachedWebBrowserInfo;

  /// Information derived from `Navigator`.
  Future<WebBrowserInfo> get webBrowserInfo async =>
      _cachedWebBrowserInfo ??= await _platform.webBrowserInfo();

  /// This information does not change from call to call. Cache it.
  MacOsDeviceInfo? _cachedMacosDeviceInfo;

  /// Returns device information for macos. Information sourced from Sysctl.
  Future<MacOsDeviceInfo> get macOsInfo async =>
      _cachedMacosDeviceInfo ??= await _platform.macosInfo();

  WindowsDeviceInfo? _cachedWindowsDeviceInfo;

  /// Returns device information for Windows.
  Future<WindowsDeviceInfo> get windowsInfo async =>
      _cachedWindowsDeviceInfo ??= await _platform.windowsInfo()!;

  /// Returns device information for the current platform.
  Future<BaseDeviceInfo> get deviceInfo async {
    if (kIsWeb) {
      return webBrowserInfo;
    } else {
      if (Platform.isAndroid) {
        return androidInfo;
      } else if (Platform.isIOS) {
        return iosInfo;
      } else if (Platform.isLinux) {
        return linuxInfo;
      } else if (Platform.isMacOS) {
        return macOsInfo;
      } else if (Platform.isWindows) {
        return windowsInfo;
      }
    }

    throw UnsupportedError('Unsupported platform');
  }
}
```

```dart
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  group('SharedPreferences', () {
    const Map<String, Object> kTestValues = <String, Object>{
      'flutter.String': 'hello world',
      'flutter.bool': true,
      'flutter.int': 42,
      'flutter.double': 3.14159,
      'flutter.List': <String>['foo', 'bar'],
    };

    const Map<String, dynamic> kTestValues2 = <String, dynamic>{
      'flutter.String': 'goodbye world',
      'flutter.bool': false,
      'flutter.int': 1337,
      'flutter.double': 2.71828,
      'flutter.List': <String>['baz', 'quox'],
    };

    late FakeSharedPreferencesStore store;
    late SharedPreferences preferences;

    setUp(() async {
      store = FakeSharedPreferencesStore(kTestValues);
      SharedPreferencesStorePlatform.instance = store;
      preferences = await SharedPreferences.getInstance();
      store.log.clear();
    });

    tearDown(() async {
      await preferences.clear();
      await store.clear();
    });

    test('reading', () async {
      expect(preferences.get('String'), kTestValues['flutter.String']);
      expect(preferences.get('bool'), kTestValues['flutter.bool']);
      expect(preferences.get('int'), kTestValues['flutter.int']);
      expect(preferences.get('double'), kTestValues['flutter.double']);
      expect(preferences.get('List'), kTestValues['flutter.List']);
      expect(preferences.getString('String'), kTestValues['flutter.String']);
      expect(preferences.getBool('bool'), kTestValues['flutter.bool']);
      expect(preferences.getInt('int'), kTestValues['flutter.int']);
      expect(preferences.getDouble('double'), kTestValues['flutter.double']);
      expect(preferences.getStringList('List'), kTestValues['flutter.List']);
      expect(store.log, <Matcher>[]);
    });

    test('writing', () async {
      await Future.wait(<Future<bool>>[
        preferences.setString('String', kTestValues2['flutter.String']),
        preferences.setBool('bool', kTestValues2['flutter.bool']),
        preferences.setInt('int', kTestValues2['flutter.int']),
        preferences.setDouble('double', kTestValues2['flutter.double']),
        preferences.setStringList('List', kTestValues2['flutter.List'])
      ]);
      expect(
        store.log,
        <Matcher>[
          isMethodCall('setValue', arguments: <dynamic>[
            'String',
            'flutter.String',
            kTestValues2['flutter.String'],
          ]),
          isMethodCall('setValue', arguments: <dynamic>[
            'Bool',
            'flutter.bool',
            kTestValues2['flutter.bool'],
          ]),
          isMethodCall('setValue', arguments: <dynamic>[
            'Int',
            'flutter.int',
            kTestValues2['flutter.int'],
          ]),
          isMethodCall('setValue', arguments: <dynamic>[
            'Double',
            'flutter.double',
            kTestValues2['flutter.double'],
          ]),
          isMethodCall('setValue', arguments: <dynamic>[
            'StringList',
            'flutter.List',
            kTestValues2['flutter.List'],
          ]),
        ],
      );
      store.log.clear();

      expect(preferences.getString('String'), kTestValues2['flutter.String']);
      expect(preferences.getBool('bool'), kTestValues2['flutter.bool']);
      expect(preferences.getInt('int'), kTestValues2['flutter.int']);
      expect(preferences.getDouble('double'), kTestValues2['flutter.double']);
      expect(preferences.getStringList('List'), kTestValues2['flutter.List']);
      expect(store.log, equals(<MethodCall>[]));
    });

    test('removing', () async {
      const String key = 'testKey';
      await preferences.remove(key);
      expect(
          store.log,
          List<Matcher>.filled(
            1,
            isMethodCall(
              'remove',
              arguments: 'flutter.$key',
            ),
            growable: true,
          ));
    });

    test('containsKey', () async {
      const String key = 'testKey';

      expect(false, preferences.containsKey(key));

      await preferences.setString(key, 'test');
      expect(true, preferences.containsKey(key));
    });

    test('clearing', () async {
      await preferences.clear();
      expect(preferences.getString('String'), null);
      expect(preferences.getBool('bool'), null);
      expect(preferences.getInt('int'), null);
      expect(preferences.getDouble('double'), null);
      expect(preferences.getStringList('List'), null);
      expect(store.log, <Matcher>[isMethodCall('clear', arguments: null)]);
    });

    test('reloading', () async {
      await preferences.setString(
          'String', kTestValues['flutter.String'] as String);
      expect(preferences.getString('String'), kTestValues['flutter.String']);

      SharedPreferences.setMockInitialValues(
          kTestValues2.cast<String, Object>());
      expect(preferences.getString('String'), kTestValues['flutter.String']);

      await preferences.reload();
      expect(preferences.getString('String'), kTestValues2['flutter.String']);
    });

    test('back to back calls should return same instance.', () async {
      final Future<SharedPreferences> first = SharedPreferences.getInstance();
      final Future<SharedPreferences> second = SharedPreferences.getInstance();
      expect(await first, await second);
    });

    test('string list type is dynamic (usually from method channel)', () async {
      SharedPreferences.setMockInitialValues(<String, Object>{
        'dynamic_list': <dynamic>['1', '2']
      });
      final SharedPreferences prefs = await SharedPreferences.getInstance();
      final List<String>? value = prefs.getStringList('dynamic_list');
      expect(value, <String>['1', '2']);
    });

    group('mocking', () {
      const String _key = 'dummy';
      const String _prefixedKey = 'flutter.' + _key;

      test('test 1', () async {
        SharedPreferences.setMockInitialValues(
            <String, Object>{_prefixedKey: 'my string'});
        final SharedPreferences prefs = await SharedPreferences.getInstance();
        final String? value = prefs.getString(_key);
        expect(value, 'my string');
      });

      test('test 2', () async {
        SharedPreferences.setMockInitialValues(
            <String, Object>{_prefixedKey: 'my other string'});
        final SharedPreferences prefs = await SharedPreferences.getInstance();
        final String? value = prefs.getString(_key);
        expect(value, 'my other string');
      });
    });

    test('writing copy of strings list', () async {
      final List<String> myList = <String>[];
      await preferences.setStringList("myList", myList);
      myList.add("foobar");

      final List<String> cachedList = preferences.getStringList('myList')!;
      expect(cachedList, <String>[]);

      cachedList.add("foobar2");

      expect(preferences.getStringList('myList'), <String>[]);
    });
  });

  test('calling mock initial values with non-prefixed keys succeeds', () async {
    SharedPreferences.setMockInitialValues(<String, Object>{
      'test': 'foo',
    });
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    final String? value = prefs.getString('test');
    expect(value, 'foo');
  });
}

class FakeSharedPreferencesStore implements SharedPreferencesStorePlatform {
  FakeSharedPreferencesStore(Map<String, Object> data)
      : backend = InMemorySharedPreferencesStore.withData(data);

  final InMemorySharedPreferencesStore backend;
  final List<MethodCall> log = <MethodCall>[];

  @override
  bool get isMock => true;

  @override
  Future<bool> clear() {
    log.add(MethodCall('clear'));
    return backend.clear();
  }

  @override
  Future<Map<String, Object>> getAll() {
    log.add(MethodCall('getAll'));
    return backend.getAll();
  }

  @override
  Future<bool> remove(String key) {
    log.add(MethodCall('remove', key));
    return backend.remove(key);
  }

  @override
  Future<bool> setValue(String valueType, String key, Object value) {
    log.add(MethodCall('setValue', <dynamic>[valueType, key, value]));
    return backend.setValue(valueType, key, value);
  }
}
```

```dart
// Copyright 2019, the Chromium project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

// @dart=2.9

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter_driver/flutter_driver.dart';

Future<void> main() async {
  final FlutterDriver driver = await FlutterDriver.connect();
  final String data =
      await driver.requestData(null, timeout: const Duration(minutes: 1));
  await driver.close();
  final Map<String, dynamic> result = jsonDecode(data);
  exit(result['result'] == 'true' ? 0 : 1);
}
```

```dart
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// @dart=2.9

import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('$SharedPreferences', () {
    const Map<String, dynamic> kTestValues = <String, dynamic>{
      'flutter.String': 'hello world',
      'flutter.bool': true,
      'flutter.int': 42,
      'flutter.double': 3.14159,
      'flutter.List': <String>['foo', 'bar'],
    };

    const Map<String, dynamic> kTestValues2 = <String, dynamic>{
      'flutter.String': 'goodbye world',
      'flutter.bool': false,
      'flutter.int': 1337,
      'flutter.double': 2.71828,
      'flutter.List': <String>['baz', 'quox'],
    };

    SharedPreferences preferences;

    setUp(() async {
      preferences = await SharedPreferences.getInstance();
    });

    tearDown(() {
      preferences.clear();
    });

    testWidgets('reading', (WidgetTester _) async {
      expect(preferences.get('String'), isNull);
      expect(preferences.get('bool'), isNull);
      expect(preferences.get('int'), isNull);
      expect(preferences.get('double'), isNull);
      expect(preferences.get('List'), isNull);
      expect(preferences.getString('String'), isNull);
      expect(preferences.getBool('bool'), isNull);
      expect(preferences.getInt('int'), isNull);
      expect(preferences.getDouble('double'), isNull);
      expect(preferences.getStringList('List'), isNull);
    });

    testWidgets('writing', (WidgetTester _) async {
      await Future.wait(<Future<bool>>[
        preferences.setString('String', kTestValues2['flutter.String']),
        preferences.setBool('bool', kTestValues2['flutter.bool']),
        preferences.setInt('int', kTestValues2['flutter.int']),
        preferences.setDouble('double', kTestValues2['flutter.double']),
        preferences.setStringList('List', kTestValues2['flutter.List'])
      ]);
      expect(preferences.getString('String'), kTestValues2['flutter.String']);
      expect(preferences.getBool('bool'), kTestValues2['flutter.bool']);
      expect(preferences.getInt('int'), kTestValues2['flutter.int']);
      expect(preferences.getDouble('double'), kTestValues2['flutter.double']);
      expect(preferences.getStringList('List'), kTestValues2['flutter.List']);
    });

    testWidgets('removing', (WidgetTester _) async {
      const String key = 'testKey';
      await preferences.setString(key, kTestValues['flutter.String']);
      await preferences.setBool(key, kTestValues['flutter.bool']);
      await preferences.setInt(key, kTestValues['flutter.int']);
      await preferences.setDouble(key, kTestValues['flutter.double']);
      await preferences.setStringList(key, kTestValues['flutter.List']);
      await preferences.remove(key);
      expect(preferences.get('testKey'), isNull);
    });

    testWidgets('clearing', (WidgetTester _) async {
      await preferences.setString('String', kTestValues['flutter.String']);
      await preferences.setBool('bool', kTestValues['flutter.bool']);
      await preferences.setInt('int', kTestValues['flutter.int']);
      await preferences.setDouble('double', kTestValues['flutter.double']);
      await preferences.setStringList('List', kTestValues['flutter.List']);
      await preferences.clear();
      expect(preferences.getString('String'), null);
      expect(preferences.getBool('bool'), null);
      expect(preferences.getInt('int'), null);
      expect(preferences.getDouble('double'), null);
      expect(preferences.getStringList('List'), null);
    });

    testWidgets('simultaneous writes', (WidgetTester _) async {
      final List<Future<bool>> writes = <Future<bool>>[];
      final int writeCount = 100;
      for (int i = 1; i <= writeCount; i++) {
        writes.add(preferences.setInt('int', i));
      }
      List<bool> result = await Future.wait(writes, eagerError: true);
      // All writes should succeed.
      expect(result.where((element) => !element), isEmpty);
      // The last write should win.
      expect(preferences.getInt('int'), writeCount);
    });
  });
}
```

```dart
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ignore_for_file: public_member_api_docs

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SharedPreferences Demo',
      home: SharedPreferencesDemo(),
    );
  }
}

class SharedPreferencesDemo extends StatefulWidget {
  SharedPreferencesDemo({Key? key}) : super(key: key);

  @override
  SharedPreferencesDemoState createState() => SharedPreferencesDemoState();
}

class SharedPreferencesDemoState extends State<SharedPreferencesDemo> {
  Future<SharedPreferences> _prefs = SharedPreferences.getInstance();
  late Future<int> _counter;

  Future<void> _incrementCounter() async {
    final SharedPreferences prefs = await _prefs;
    final int counter = (prefs.getInt('counter') ?? 0) + 1;

    setState(() {
      _counter = prefs.setInt("counter", counter).then((bool success) {
        return counter;
      });
    });
  }

  @override
  void initState() {
    super.initState();
    _counter = _prefs.then((SharedPreferences prefs) {
      return (prefs.getInt('counter') ?? 0);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("SharedPreferences Demo"),
      ),
      body: Center(
          child: FutureBuilder<int>(
              future: _counter,
              builder: (BuildContext context, AsyncSnapshot<int> snapshot) {
                switch (snapshot.connectionState) {
                  case ConnectionState.waiting:
                    return const CircularProgressIndicator();
                  default:
                    if (snapshot.hasError) {
                      return Text('Error: ${snapshot.error}');
                    } else {
                      return Text(
                        'Button tapped ${snapshot.data} time${snapshot.data == 1 ? '' : 's'}.\n\n'
                        'This should persist across restarts.',
                      );
                    }
                }
              })),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
```

```dart
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:io' show Platform;

import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:meta/meta.dart';
import 'package:shared_preferences_linux/shared_preferences_linux.dart';
import 'package:shared_preferences_platform_interface/method_channel_shared_preferences.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_platform_interface.dart';
import 'package:shared_preferences_windows/shared_preferences_windows.dart';

/// Wraps NSUserDefaults (on iOS) and SharedPreferences (on Android), providing
/// a persistent store for simple data.
///
/// Data is persisted to disk asynchronously.
class SharedPreferences {
  SharedPreferences._(this._preferenceCache);

  static const String _prefix = 'flutter.';
  static Completer<SharedPreferences>? _completer;
  static bool _manualDartRegistrationNeeded = true;

  static SharedPreferencesStorePlatform get _store {
    // This is to manually endorse the Linux implementation until automatic
    // registration of dart plugins is implemented. For details see
    // https://github.com/flutter/flutter/issues/52267.
    if (_manualDartRegistrationNeeded) {
      // Only do the initial registration if it hasn't already been overridden
      // with a non-default instance.
      if (!kIsWeb &&
          SharedPreferencesStorePlatform.instance
              is MethodChannelSharedPreferencesStore) {
        if (Platform.isLinux) {
          SharedPreferencesStorePlatform.instance = SharedPreferencesLinux();
        } else if (Platform.isWindows) {
          SharedPreferencesStorePlatform.instance = SharedPreferencesWindows();
        }
      }
      _manualDartRegistrationNeeded = false;
    }

    return SharedPreferencesStorePlatform.instance;
  }

  /// Loads and parses the [SharedPreferences] for this app from disk.
  ///
  /// Because this is reading from disk, it shouldn't be awaited in
  /// performance-sensitive blocks.
  static Future<SharedPreferences> getInstance() async {
    if (_completer == null) {
      final completer = Completer<SharedPreferences>();
      try {
        final Map<String, Object> preferencesMap =
            await _getSharedPreferencesMap();
        completer.complete(SharedPreferences._(preferencesMap));
      } on Exception catch (e) {
        // If there's an error, explicitly return the future with an error.
        // then set the completer to null so we can retry.
        completer.completeError(e);
        final Future<SharedPreferences> sharedPrefsFuture = completer.future;
        _completer = null;
        return sharedPrefsFuture;
      }
      _completer = completer;
    }
    return _completer!.future;
  }

  /// The cache that holds all preferences.
  ///
  /// It is instantiated to the current state of the SharedPreferences or
  /// NSUserDefaults object and then kept in sync via setter methods in this
  /// class.
  ///
  /// It is NOT guaranteed that this cache and the device prefs will remain
  /// in sync since the setter method might fail for any reason.
  final Map<String, Object> _preferenceCache;

  /// Returns all keys in the persistent storage.
  Set<String> getKeys() => Set<String>.from(_preferenceCache.keys);

  /// Reads a value of any type from persistent storage.
  Object? get(String key) => _preferenceCache[key];

  /// Reads a value from persistent storage, throwing an exception if it's not a
  /// bool.
  bool? getBool(String key) => _preferenceCache[key] as bool?;

  /// Reads a value from persistent storage, throwing an exception if it's not
  /// an int.
  int? getInt(String key) => _preferenceCache[key] as int?;

  /// Reads a value from persistent storage, throwing an exception if it's not a
  /// double.
  double? getDouble(String key) => _preferenceCache[key] as double?;

  /// Reads a value from persistent storage, throwing an exception if it's not a
  /// String.
  String? getString(String key) => _preferenceCache[key] as String?;

  /// Returns true if persistent storage the contains the given [key].
  bool containsKey(String key) => _preferenceCache.containsKey(key);

  /// Reads a set of string values from persistent storage, throwing an
  /// exception if it's not a string set.
  List<String>? getStringList(String key) {
    List<dynamic>? list = _preferenceCache[key] as List<dynamic>?;
    if (list != null && list is! List<String>) {
      list = list.cast<String>().toList();
      _preferenceCache[key] = list;
    }
    // Make a copy of the list so that later mutations won't propagate
    return list?.toList() as List<String>?;
  }

  /// Saves a boolean [value] to persistent storage in the background.
  Future<bool> setBool(String key, bool value) => _setValue('Bool', key, value);

  /// Saves an integer [value] to persistent storage in the background.
  Future<bool> setInt(String key, int value) => _setValue('Int', key, value);

  /// Saves a double [value] to persistent storage in the background.
  ///
  /// Android doesn't support storing doubles, so it will be stored as a float.
  Future<bool> setDouble(String key, double value) =>
      _setValue('Double', key, value);

  /// Saves a string [value] to persistent storage in the background.
  Future<bool> setString(String key, String value) =>
      _setValue('String', key, value);

  /// Saves a list of strings [value] to persistent storage in the background.
  Future<bool> setStringList(String key, List<String> value) =>
      _setValue('StringList', key, value);

  /// Removes an entry from persistent storage.
  Future<bool> remove(String key) {
    final String prefixedKey = '$_prefix$key';
    _preferenceCache.remove(key);
    return _store.remove(prefixedKey);
  }

  Future<bool> _setValue(String valueType, String key, Object value) {
    ArgumentError.checkNotNull(value, 'value');
    final String prefixedKey = '$_prefix$key';
    if (value is List<String>) {
      // Make a copy of the list so that later mutations won't propagate
      _preferenceCache[key] = value.toList();
    } else {
      _preferenceCache[key] = value;
    }
    return _store.setValue(valueType, prefixedKey, value);
  }

  /// Always returns true.
  /// On iOS, synchronize is marked deprecated. On Android, we commit every set.
  @deprecated
  Future<bool> commit() async => true;

  /// Completes with true once the user preferences for the app has been cleared.
  Future<bool> clear() {
    _preferenceCache.clear();
    return _store.clear();
  }

  /// Fetches the latest values from the host platform.
  ///
  /// Use this method to observe modifications that were made in native code
  /// (without using the plugin) while the app is running.
  Future<void> reload() async {
    final Map<String, Object> preferences =
        await SharedPreferences._getSharedPreferencesMap();
    _preferenceCache.clear();
    _preferenceCache.addAll(preferences);
  }

  static Future<Map<String, Object>> _getSharedPreferencesMap() async {
    final Map<String, Object> fromSystem = await _store.getAll();
    assert(fromSystem != null);
    // Strip the flutter. prefix from the returned preferences.
    final Map<String, Object> preferencesMap = <String, Object>{};
    for (String key in fromSystem.keys) {
      assert(key.startsWith(_prefix));
      preferencesMap[key.substring(_prefix.length)] = fromSystem[key]!;
    }
    return preferencesMap;
  }

  /// Initializes the shared preferences with mock values for testing.
  ///
  /// If the singleton instance has been initialized already, it is nullified.
  @visibleForTesting
  static void setMockInitialValues(Map<String, Object> values) {
    final Map<String, Object> newValues =
        values.map<String, Object>((String key, Object value) {
      String newKey = key;
      if (!key.startsWith(_prefix)) {
        newKey = '$_prefix$key';
      }
      return MapEntry<String, Object>(newKey, value);
    });
    SharedPreferencesStorePlatform.instance =
        InMemorySharedPreferencesStore.withData(newValues);
    _completer = null;
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
// Autogenerated from Pigeon (v9.2.4), do not edit directly.
// See also: https://pub.dev/packages/pigeon
// ignore_for_file: public_member_api_docs, non_constant_identifier_names, avoid_as, unused_import, unnecessary_parenthesis, unnecessary_import
// ignore_for_file: avoid_relative_lib_imports
import 'dart:async';
import 'dart:typed_data' show Float64List, Int32List, Int64List, Uint8List;
import 'package:flutter/foundation.dart' show ReadBuffer, WriteBuffer;
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:path_provider_foundation/messages.g.dart';

abstract class TestPathProviderApi {
  static TestDefaultBinaryMessengerBinding? get _testBinaryMessengerBinding =>
      TestDefaultBinaryMessengerBinding.instance;
  static const MessageCodec<Object?> codec = StandardMessageCodec();

  String? getDirectoryPath(DirectoryType type);

  String? getContainerPath(String appGroupIdentifier);

  static void setup(TestPathProviderApi? api,
      {BinaryMessenger? binaryMessenger}) {
    {
      final BasicMessageChannel<Object?> channel = BasicMessageChannel<Object?>(
          'dev.flutter.pigeon.PathProviderApi.getDirectoryPath', codec,
          binaryMessenger: binaryMessenger);
      if (api == null) {
        _testBinaryMessengerBinding!.defaultBinaryMessenger
            .setMockDecodedMessageHandler<Object?>(channel, null);
      } else {
        _testBinaryMessengerBinding!.defaultBinaryMessenger
            .setMockDecodedMessageHandler<Object?>(channel,
                (Object? message) async {
          assert(message != null,
              'Argument for dev.flutter.pigeon.PathProviderApi.getDirectoryPath was null.');
          final List<Object?> args = (message as List<Object?>?)!;
          final DirectoryType? arg_type =
              args[0] == null ? null : DirectoryType.values[args[0] as int];
          assert(arg_type != null,
              'Argument for dev.flutter.pigeon.PathProviderApi.getDirectoryPath was null, expected non-null DirectoryType.');
          final String? output = api.getDirectoryPath(arg_type!);
          return <Object?>[output];
        });
      }
    }
    {
      final BasicMessageChannel<Object?> channel = BasicMessageChannel<Object?>(
          'dev.flutter.pigeon.PathProviderApi.getContainerPath', codec,
          binaryMessenger: binaryMessenger);
      if (api == null) {
        _testBinaryMessengerBinding!.defaultBinaryMessenger
            .setMockDecodedMessageHandler<Object?>(channel, null);
      } else {
        _testBinaryMessengerBinding!.defaultBinaryMessenger
            .setMockDecodedMessageHandler<Object?>(channel,
                (Object? message) async {
          assert(message != null,
              'Argument for dev.flutter.pigeon.PathProviderApi.getContainerPath was null.');
          final List<Object?> args = (message as List<Object?>?)!;
          final String? arg_appGroupIdentifier = (args[0] as String?);
          assert(arg_appGroupIdentifier != null,
              'Argument for dev.flutter.pigeon.PathProviderApi.getContainerPath was null, expected non-null String.');
          final String? output = api.getContainerPath(arg_appGroupIdentifier!);
          return <Object?>[output];
        });
      }
    }
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider_foundation/messages.g.dart';
import 'package:path_provider_foundation/path_provider_foundation.dart';

import 'messages_test.g.dart';
import 'path_provider_foundation_test.mocks.dart';

@GenerateMocks(<Type>[TestPathProviderApi])
void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  group('PathProviderFoundation', () {
    late MockTestPathProviderApi mockApi;
    // These unit tests use the actual filesystem, since an injectable
    // filesystem would add a runtime dependency to the package, so everything
    // is contained to a temporary directory.
    late Directory testRoot;

    setUp(() async {
      testRoot = Directory.systemTemp.createTempSync();
      mockApi = MockTestPathProviderApi();
      TestPathProviderApi.setup(mockApi);
    });

    tearDown(() {
      testRoot.deleteSync(recursive: true);
    });

    test('getTemporaryPath', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      final String temporaryPath = p.join(testRoot.path, 'temporary', 'path');
      when(mockApi.getDirectoryPath(DirectoryType.temp))
          .thenReturn(temporaryPath);

      final String? path = await pathProvider.getTemporaryPath();

      verify(mockApi.getDirectoryPath(DirectoryType.temp));
      expect(path, temporaryPath);
    });

    test('getApplicationSupportPath', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      final String applicationSupportPath =
          p.join(testRoot.path, 'application', 'support', 'path');
      when(mockApi.getDirectoryPath(DirectoryType.applicationSupport))
          .thenReturn(applicationSupportPath);

      final String? path = await pathProvider.getApplicationSupportPath();

      verify(mockApi.getDirectoryPath(DirectoryType.applicationSupport));
      expect(path, applicationSupportPath);
    });

    test('getApplicationSupportPath creates the directory if necessary',
        () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      final String applicationSupportPath =
          p.join(testRoot.path, 'application', 'support', 'path');
      when(mockApi.getDirectoryPath(DirectoryType.applicationSupport))
          .thenReturn(applicationSupportPath);

      final String? path = await pathProvider.getApplicationSupportPath();

      expect(Directory(path!).existsSync(), isTrue);
    });

    test('getLibraryPath', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      final String libraryPath = p.join(testRoot.path, 'library', 'path');
      when(mockApi.getDirectoryPath(DirectoryType.library))
          .thenReturn(libraryPath);

      final String? path = await pathProvider.getLibraryPath();

      verify(mockApi.getDirectoryPath(DirectoryType.library));
      expect(path, libraryPath);
    });

    test('getApplicationDocumentsPath', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      final String applicationDocumentsPath =
          p.join(testRoot.path, 'application', 'documents', 'path');
      when(mockApi.getDirectoryPath(DirectoryType.applicationDocuments))
          .thenReturn(applicationDocumentsPath);

      final String? path = await pathProvider.getApplicationDocumentsPath();

      verify(mockApi.getDirectoryPath(DirectoryType.applicationDocuments));
      expect(path, applicationDocumentsPath);
    });

    test('getDownloadsPath', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      final String downloadsPath = p.join(testRoot.path, 'downloads', 'path');
      when(mockApi.getDirectoryPath(DirectoryType.downloads))
          .thenReturn(downloadsPath);

      final String? result = await pathProvider.getDownloadsPath();

      verify(mockApi.getDirectoryPath(DirectoryType.downloads));
      expect(result, downloadsPath);
    });

    test('getExternalCachePaths throws', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      expect(pathProvider.getExternalCachePaths(), throwsA(isUnsupportedError));
    });

    test('getExternalStoragePath throws', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      expect(
          pathProvider.getExternalStoragePath(), throwsA(isUnsupportedError));
    });

    test('getExternalStoragePaths throws', () async {
      final PathProviderFoundation pathProvider = PathProviderFoundation();
      expect(
          pathProvider.getExternalStoragePaths(), throwsA(isUnsupportedError));
    });

    test('getContainerPath', () async {
      final PathProviderFoundation pathProvider =
          PathProviderFoundation(platform: FakePlatformProvider(isIOS: true));
      const String appGroupIdentifier = 'group.example.test';

      final String containerPath = p.join(testRoot.path, 'container', 'path');
      when(mockApi.getContainerPath(appGroupIdentifier))
          .thenReturn(containerPath);

      final String? result = await pathProvider.getContainerPath(
          appGroupIdentifier: appGroupIdentifier);

      verify(mockApi.getContainerPath(appGroupIdentifier));
      expect(result, containerPath);
    });

    test('getContainerPath throws on macOS', () async {
      final PathProviderFoundation pathProvider =
          PathProviderFoundation(platform: FakePlatformProvider(isIOS: false));
      expect(
          pathProvider.getContainerPath(
              appGroupIdentifier: 'group.example.test'),
          throwsA(isUnsupportedError));
    });
  });
}

/// Fake implementation of PathProviderPlatformProvider that returns iOS is true
class FakePlatformProvider implements PathProviderPlatformProvider {
  FakePlatformProvider({required this.isIOS});
  @override
  bool isIOS;
}
```

```dart
// Mocks generated by Mockito 5.4.0 from annotations
// in path_provider_foundation/test/path_provider_foundation_test.dart.
// Do not manually edit this file.

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'package:mockito/mockito.dart' as _i1;
import 'package:path_provider_foundation/messages.g.dart' as _i3;

import 'messages_test.g.dart' as _i2;

// ignore_for_file: type=lint
// ignore_for_file: avoid_redundant_argument_values
// ignore_for_file: avoid_setters_without_getters
// ignore_for_file: comment_references
// ignore_for_file: implementation_imports
// ignore_for_file: invalid_use_of_visible_for_testing_member
// ignore_for_file: prefer_const_constructors
// ignore_for_file: unnecessary_parenthesis
// ignore_for_file: camel_case_types
// ignore_for_file: subtype_of_sealed_class

/// A class which mocks [TestPathProviderApi].
///
/// See the documentation for Mockito's code generation for more information.
class MockTestPathProviderApi extends _i1.Mock
    implements _i2.TestPathProviderApi {
  MockTestPathProviderApi() {
    _i1.throwOnMissingStub(this);
  }

  @override
  String? getDirectoryPath(_i3.DirectoryType? type) =>
      (super.noSuchMethod(Invocation.method(
        #getDirectoryPath,
        [type],
      )) as String?);
  @override
  String? getContainerPath(String? appGroupIdentifier) =>
      (super.noSuchMethod(Invocation.method(
        #getContainerPath,
        [appGroupIdentifier],
      )) as String?);
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider_foundation/path_provider_foundation.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('getTemporaryDirectory', (WidgetTester tester) async {
    final PathProviderPlatform provider = PathProviderPlatform.instance;
    final String? result = await provider.getTemporaryPath();
    _verifySampleFile(result, 'temporaryDirectory');
  });

  testWidgets('getApplicationDocumentsDirectory', (WidgetTester tester) async {
    final PathProviderPlatform provider = PathProviderPlatform.instance;
    final String? result = await provider.getApplicationDocumentsPath();
    _verifySampleFile(result, 'applicationDocuments');
  });

  testWidgets('getApplicationSupportDirectory', (WidgetTester tester) async {
    final PathProviderPlatform provider = PathProviderPlatform.instance;
    final String? result = await provider.getApplicationSupportPath();
    _verifySampleFile(result, 'applicationSupport');
  });

  testWidgets('getLibraryDirectory', (WidgetTester tester) async {
    final PathProviderPlatform provider = PathProviderPlatform.instance;
    final String? result = await provider.getLibraryPath();
    _verifySampleFile(result, 'library');
  });

  testWidgets('getDownloadsDirectory', (WidgetTester tester) async {
    final PathProviderPlatform provider = PathProviderPlatform.instance;
    final String? result = await provider.getDownloadsPath();
    // _verifySampleFile causes hangs in driver for some reason, so just
    // validate that a non-empty path was returned.
    expect(result, isNotEmpty);
  });

  testWidgets('getContainerDirectory', (WidgetTester tester) async {
    if (Platform.isIOS) {
      final PathProviderFoundation provider = PathProviderFoundation();
      final String? result = await provider.getContainerPath(
          appGroupIdentifier: 'group.flutter.appGroupTest');
      _verifySampleFile(result, 'appGroup');
    }
  });
}

/// Verify a file called [name] in [directoryPath] by recreating it with test
/// contents when necessary.
///
/// If [createDirectory] is true, the directory will be created if missing.
void _verifySampleFile(String? directoryPath, String name) {
  expect(directoryPath, isNotNull);
  if (directoryPath == null) {
    return;
  }
  final Directory directory = Directory(directoryPath);
  final File file = File('${directory.path}${Platform.pathSeparator}$name');

  if (file.existsSync()) {
    file.deleteSync();
    expect(file.existsSync(), isFalse);
  }

  file.writeAsStringSync('Hello world!');
  expect(file.readAsStringSync(), 'Hello world!');
  expect(directory.listSync(), isNotEmpty);
  file.deleteSync();
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ignore_for_file: public_member_api_docs

import 'package:flutter/material.dart';
import 'package:path_provider_foundation/path_provider_foundation.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';

void main() {
  runApp(const MyApp());
}

/// Sample app
class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String? _tempDirectory = 'Unknown';
  String? _downloadsDirectory = 'Unknown';
  String? _libraryDirectory = 'Unknown';
  String? _appSupportDirectory = 'Unknown';
  String? _documentsDirectory = 'Unknown';
  String? _containerDirectory = 'Unknown';

  @override
  void initState() {
    super.initState();
    initDirectories();
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initDirectories() async {
    String? tempDirectory;
    String? downloadsDirectory;
    String? appSupportDirectory;
    String? libraryDirectory;
    String? documentsDirectory;
    String? containerDirectory;
    final PathProviderPlatform provider = PathProviderPlatform.instance;
    final PathProviderFoundation providerFoundation = PathProviderFoundation();

    try {
      tempDirectory = await provider.getTemporaryPath();
    } catch (exception) {
      tempDirectory = 'Failed to get temp directory: $exception';
    }
    try {
      downloadsDirectory = await provider.getDownloadsPath();
    } catch (exception) {
      downloadsDirectory = 'Failed to get downloads directory: $exception';
    }

    try {
      documentsDirectory = await provider.getApplicationDocumentsPath();
    } catch (exception) {
      documentsDirectory = 'Failed to get documents directory: $exception';
    }

    try {
      libraryDirectory = await provider.getLibraryPath();
    } catch (exception) {
      libraryDirectory = 'Failed to get library directory: $exception';
    }

    try {
      appSupportDirectory = await provider.getApplicationSupportPath();
    } catch (exception) {
      appSupportDirectory = 'Failed to get app support directory: $exception';
    }

    try {
      containerDirectory = await providerFoundation.getContainerPath(
          appGroupIdentifier: 'group.flutter.appGroupTest');
    } catch (exception) {
      containerDirectory =
          'Failed to get app group container directory: $exception';
    }

    setState(() {
      _tempDirectory = tempDirectory;
      _downloadsDirectory = downloadsDirectory;
      _libraryDirectory = libraryDirectory;
      _appSupportDirectory = appSupportDirectory;
      _documentsDirectory = documentsDirectory;
      _containerDirectory = containerDirectory;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Path Provider example app'),
        ),
        body: Center(
          child: Column(
            children: <Widget>[
              Text('Temp Directory: $_tempDirectory\n'),
              Text('Documents Directory: $_documentsDirectory\n'),
              Text('Downloads Directory: $_downloadsDirectory\n'),
              Text('Library Directory: $_libraryDirectory\n'),
              Text('Application Support Directory: $_appSupportDirectory\n'),
              Text('App Group Container Directory: $_containerDirectory\n'),
            ],
          ),
        ),
      ),
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
import 'package:pigeon/pigeon.dart';

@ConfigurePigeon(PigeonOptions(
  input: 'pigeons/messages.dart',
  swiftOut: 'macos/Classes/messages.g.swift',
  dartOut: 'lib/messages.g.dart',
  dartTestOut: 'test/messages_test.g.dart',
  copyrightHeader: 'pigeons/copyright.txt',
))
enum DirectoryType {
  applicationDocuments,
  applicationSupport,
  downloads,
  library,
  temp,
}

@HostApi(dartHostTestHandler: 'TestPathProviderApi')
abstract class PathProviderApi {
  String? getDirectoryPath(DirectoryType type);
  String? getContainerPath(String appGroupIdentifier);
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
// Autogenerated from Pigeon (v9.2.4), do not edit directly.
// See also: https://pub.dev/packages/pigeon
// ignore_for_file: public_member_api_docs, non_constant_identifier_names, avoid_as, unused_import, unnecessary_parenthesis, prefer_null_aware_operators, omit_local_variable_types, unused_shown_name, unnecessary_import

import 'dart:async';
import 'dart:typed_data' show Float64List, Int32List, Int64List, Uint8List;

import 'package:flutter/foundation.dart' show ReadBuffer, WriteBuffer;
import 'package:flutter/services.dart';

enum DirectoryType {
  applicationDocuments,
  applicationSupport,
  downloads,
  library,
  temp,
}

class PathProviderApi {
  /// Constructor for [PathProviderApi].  The [binaryMessenger] named argument is
  /// available for dependency injection.  If it is left null, the default
  /// BinaryMessenger will be used which routes to the host platform.
  PathProviderApi({BinaryMessenger? binaryMessenger})
      : _binaryMessenger = binaryMessenger;
  final BinaryMessenger? _binaryMessenger;

  static const MessageCodec<Object?> codec = StandardMessageCodec();

  Future<String?> getDirectoryPath(DirectoryType arg_type) async {
    final BasicMessageChannel<Object?> channel = BasicMessageChannel<Object?>(
        'dev.flutter.pigeon.PathProviderApi.getDirectoryPath', codec,
        binaryMessenger: _binaryMessenger);
    final List<Object?>? replyList =
        await channel.send(<Object?>[arg_type.index]) as List<Object?>?;
    if (replyList == null) {
      throw PlatformException(
        code: 'channel-error',
        message: 'Unable to establish connection on channel.',
      );
    } else if (replyList.length > 1) {
      throw PlatformException(
        code: replyList[0]! as String,
        message: replyList[1] as String?,
        details: replyList[2],
      );
    } else {
      return (replyList[0] as String?);
    }
  }

  Future<String?> getContainerPath(String arg_appGroupIdentifier) async {
    final BasicMessageChannel<Object?> channel = BasicMessageChannel<Object?>(
        'dev.flutter.pigeon.PathProviderApi.getContainerPath', codec,
        binaryMessenger: _binaryMessenger);
    final List<Object?>? replyList =
        await channel.send(<Object?>[arg_appGroupIdentifier]) as List<Object?>?;
    if (replyList == null) {
      throw PlatformException(
        code: 'channel-error',
        message: 'Unable to establish connection on channel.',
      );
    } else if (replyList.length > 1) {
      throw PlatformException(
        code: replyList[0]! as String,
        message: replyList[1] as String?,
        details: replyList[2],
      );
    } else {
      return (replyList[0] as String?);
    }
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';

import 'messages.g.dart';

/// The iOS and macOS implementation of [PathProviderPlatform].
class PathProviderFoundation extends PathProviderPlatform {
  /// Constructor that accepts a testable PathProviderPlatformProvider.
  PathProviderFoundation({
    @visibleForTesting PathProviderPlatformProvider? platform,
  }) : _platformProvider = platform ?? PathProviderPlatformProvider();

  final PathProviderPlatformProvider _platformProvider;
  final PathProviderApi _pathProvider = PathProviderApi();

  /// Registers this class as the default instance of [PathProviderPlatform]
  static void registerWith() {
    PathProviderPlatform.instance = PathProviderFoundation();
  }

  @override
  Future<String?> getTemporaryPath() {
    return _pathProvider.getDirectoryPath(DirectoryType.temp);
  }

  @override
  Future<String?> getApplicationSupportPath() async {
    final String? path =
        await _pathProvider.getDirectoryPath(DirectoryType.applicationSupport);
    if (path != null) {
      // Ensure the directory exists before returning it, for consistency with
      // other platforms.
      await Directory(path).create(recursive: true);
    }
    return path;
  }

  @override
  Future<String?> getLibraryPath() {
    return _pathProvider.getDirectoryPath(DirectoryType.library);
  }

  @override
  Future<String?> getApplicationDocumentsPath() {
    return _pathProvider.getDirectoryPath(DirectoryType.applicationDocuments);
  }

  @override
  Future<String?> getExternalStoragePath() async {
    throw UnsupportedError(
        'getExternalStoragePath is not supported on this platform');
  }

  @override
  Future<List<String>?> getExternalCachePaths() async {
    throw UnsupportedError(
        'getExternalCachePaths is not supported on this platform');
  }

  @override
  Future<List<String>?> getExternalStoragePaths({
    StorageDirectory? type,
  }) async {
    throw UnsupportedError(
        'getExternalStoragePaths is not supported on this platform');
  }

  @override
  Future<String?> getDownloadsPath() {
    return _pathProvider.getDirectoryPath(DirectoryType.downloads);
  }

  /// Returns the path to the container of the specified App Group.
  /// This is only supported for iOS.
  Future<String?> getContainerPath({required String appGroupIdentifier}) async {
    if (!_platformProvider.isIOS) {
      throw UnsupportedError(
          'getContainerPath is not supported on this platform');
    }
    return _pathProvider.getContainerPath(appGroupIdentifier);
  }
}

/// Helper class for returning information about the current platform.
@visibleForTesting
class PathProviderPlatformProvider {
  /// Specifies whether the current platform is iOS.
  bool get isIOS => Platform.isIOS;
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
import 'dart:convert';

import 'package:file/memory.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider_linux/path_provider_linux.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:shared_preferences_linux/shared_preferences_linux.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_platform_interface.dart';

void main() {
  late MemoryFileSystem fs;
  late PathProviderLinux pathProvider;

  SharedPreferencesLinux.registerWith();

  const Map<String, Object> flutterTestValues = <String, Object>{
    'flutter.String': 'hello world',
    'flutter.Bool': true,
    'flutter.Int': 42,
    'flutter.Double': 3.14159,
    'flutter.StringList': <String>['foo', 'bar'],
  };

  const Map<String, Object> prefixTestValues = <String, Object>{
    'prefix.String': 'hello world',
    'prefix.Bool': true,
    'prefix.Int': 42,
    'prefix.Double': 3.14159,
    'prefix.StringList': <String>['foo', 'bar'],
  };

  const Map<String, Object> nonPrefixTestValues = <String, Object>{
    'String': 'hello world',
    'Bool': true,
    'Int': 42,
    'Double': 3.14159,
    'StringList': <String>['foo', 'bar'],
  };

  final Map<String, Object> allTestValues = <String, Object>{};

  allTestValues.addAll(flutterTestValues);
  allTestValues.addAll(prefixTestValues);
  allTestValues.addAll(nonPrefixTestValues);

  setUp(() {
    fs = MemoryFileSystem.test();
    pathProvider = FakePathProviderLinux();
  });

  Future<String> getFilePath() async {
    final String? directory = await pathProvider.getApplicationSupportPath();
    return path.join(directory!, 'shared_preferences.json');
  }

  Future<void> writeTestFile(String value) async {
    fs.file(await getFilePath())
      ..createSync(recursive: true)
      ..writeAsStringSync(value);
  }

  Future<String> readTestFile() async {
    return fs.file(await getFilePath()).readAsStringSync();
  }

  SharedPreferencesLinux getPreferences() {
    final SharedPreferencesLinux prefs = SharedPreferencesLinux();
    prefs.fs = fs;
    prefs.pathProvider = pathProvider;
    return prefs;
  }

  test('registered instance', () {
    SharedPreferencesLinux.registerWith();
    expect(
        SharedPreferencesStorePlatform.instance, isA<SharedPreferencesLinux>());
  });

  test('getAll', () async {
    await writeTestFile(json.encode(allTestValues));
    final SharedPreferencesLinux prefs = getPreferences();

    final Map<String, Object> values = await prefs.getAll();
    expect(values, hasLength(5));
    expect(values, flutterTestValues);
  });

  test('getAllWithPrefix', () async {
    await writeTestFile(json.encode(allTestValues));
    final SharedPreferencesLinux prefs = getPreferences();

    final Map<String, Object> values = await prefs.getAllWithPrefix('prefix.');
    expect(values, hasLength(5));
    expect(values, prefixTestValues);
  });

  test('remove', () async {
    await writeTestFile('{"key1":"one","key2":2}');
    final SharedPreferencesLinux prefs = getPreferences();

    await prefs.remove('key2');

    expect(await readTestFile(), '{"key1":"one"}');
  });

  test('setValue', () async {
    await writeTestFile('{}');
    final SharedPreferencesLinux prefs = getPreferences();

    await prefs.setValue('', 'key1', 'one');
    await prefs.setValue('', 'key2', 2);

    expect(await readTestFile(), '{"key1":"one","key2":2}');
  });

  test('clear', () async {
    await writeTestFile(json.encode(flutterTestValues));
    final SharedPreferencesLinux prefs = getPreferences();

    expect(await readTestFile(), json.encode(flutterTestValues));
    await prefs.clear();
    expect(await readTestFile(), '{}');
  });

  test('clearWithPrefix', () async {
    await writeTestFile(json.encode(flutterTestValues));
    final SharedPreferencesLinux prefs = getPreferences();
    await prefs.clearWithPrefix('prefix.');
    final Map<String, Object> noValues =
        await prefs.getAllWithPrefix('prefix.');
    expect(noValues, hasLength(0));

    final Map<String, Object> values = await prefs.getAll();
    expect(values, hasLength(5));
    expect(values, flutterTestValues);
  });

  test('getAllWithNoPrefix', () async {
    await writeTestFile(json.encode(allTestValues));
    final SharedPreferencesLinux prefs = getPreferences();

    final Map<String, Object> values = await prefs.getAllWithPrefix('');
    expect(values, hasLength(15));
    expect(values, allTestValues);
  });

  test('clearWithNoPrefix', () async {
    await writeTestFile(json.encode(flutterTestValues));
    final SharedPreferencesLinux prefs = getPreferences();
    await prefs.clearWithPrefix('');
    final Map<String, Object> noValues = await prefs.getAllWithPrefix('');
    expect(noValues, hasLength(0));
  });
}

/// Fake implementation of PathProviderLinux that returns hard-coded paths,
/// allowing tests to run on any platform.
///
/// Note that this should only be used with an in-memory filesystem, as the
/// path it returns is a root path that does not actually exist on Linux.
class FakePathProviderLinux extends PathProviderPlatform
    implements PathProviderLinux {
  @override
  Future<String?> getApplicationSupportPath() async => r'/appsupport';

  @override
  Future<String?> getTemporaryPath() async => null;

  @override
  Future<String?> getLibraryPath() async => null;

  @override
  Future<String?> getApplicationDocumentsPath() async => null;

  @override
  Future<String?> getDownloadsPath() async => null;
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:shared_preferences_linux/shared_preferences_linux.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('SharedPreferencesLinux', () {
    const Map<String, Object> kTestValues = <String, Object>{
      'flutter.String': 'hello world',
      'flutter.bool': true,
      'flutter.int': 42,
      'flutter.double': 3.14159,
      'flutter.List': <String>['foo', 'bar'],
    };

    const Map<String, Object> kTestValues2 = <String, Object>{
      'flutter.String': 'goodbye world',
      'flutter.bool': false,
      'flutter.int': 1337,
      'flutter.double': 2.71828,
      'flutter.List': <String>['baz', 'quox'],
    };

    late SharedPreferencesLinux preferences;

    setUp(() async {
      preferences = SharedPreferencesLinux();
    });

    tearDown(() {
      preferences.clear();
    });

    testWidgets('reading', (WidgetTester _) async {
      final Map<String, Object> all = await preferences.getAll();
      expect(all['flutter.String'], isNull);
      expect(all['flutter.bool'], isNull);
      expect(all['flutter.int'], isNull);
      expect(all['flutter.double'], isNull);
      expect(all['flutter.List'], isNull);
    });

    testWidgets('writing', (WidgetTester _) async {
      await Future.wait(<Future<bool>>[
        preferences.setValue(
            'String', 'flutter.String', kTestValues2['flutter.String']!),
        preferences.setValue(
            'Bool', 'flutter.bool', kTestValues2['flutter.bool']!),
        preferences.setValue(
            'Int', 'flutter.int', kTestValues2['flutter.int']!),
        preferences.setValue(
            'Double', 'flutter.double', kTestValues2['flutter.double']!),
        preferences.setValue(
            'StringList', 'flutter.List', kTestValues2['flutter.List']!)
      ]);
      final Map<String, Object> all = await preferences.getAll();
      expect(all['flutter.String'], kTestValues2['flutter.String']);
      expect(all['flutter.bool'], kTestValues2['flutter.bool']);
      expect(all['flutter.int'], kTestValues2['flutter.int']);
      expect(all['flutter.double'], kTestValues2['flutter.double']);
      expect(all['flutter.List'], kTestValues2['flutter.List']);
    });

    testWidgets('removing', (WidgetTester _) async {
      const String key = 'flutter.testKey';

      await Future.wait(<Future<bool>>[
        preferences.setValue('String', key, kTestValues['flutter.String']!),
        preferences.setValue('Bool', key, kTestValues['flutter.bool']!),
        preferences.setValue('Int', key, kTestValues['flutter.int']!),
        preferences.setValue('Double', key, kTestValues['flutter.double']!),
        preferences.setValue('StringList', key, kTestValues['flutter.List']!)
      ]);
      await preferences.remove(key);
      final Map<String, Object> all = await preferences.getAll();
      expect(all[key], isNull);
    });

    testWidgets('clearing', (WidgetTester _) async {
      await Future.wait(<Future<bool>>[
        preferences.setValue(
            'String', 'flutter.String', kTestValues['flutter.String']!),
        preferences.setValue(
            'Bool', 'flutter.bool', kTestValues['flutter.bool']!),
        preferences.setValue('Int', 'flutter.int', kTestValues['flutter.int']!),
        preferences.setValue(
            'Double', 'flutter.double', kTestValues['flutter.double']!),
        preferences.setValue(
            'StringList', 'flutter.List', kTestValues['flutter.List']!)
      ]);
      await preferences.clear();
      final Map<String, Object> all = await preferences.getAll();
      expect(all['flutter.String'], null);
      expect(all['flutter.bool'], null);
      expect(all['flutter.int'], null);
      expect(all['flutter.double'], null);
      expect(all['flutter.List'], null);
    });
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ignore_for_file: public_member_api_docs

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:shared_preferences_linux/shared_preferences_linux.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      title: 'SharedPreferences Demo',
      home: SharedPreferencesDemo(),
    );
  }
}

class SharedPreferencesDemo extends StatefulWidget {
  const SharedPreferencesDemo({super.key});

  @override
  SharedPreferencesDemoState createState() => SharedPreferencesDemoState();
}

class SharedPreferencesDemoState extends State<SharedPreferencesDemo> {
  final SharedPreferencesLinux prefs = SharedPreferencesLinux();
  late Future<int> _counter;

  Future<void> _incrementCounter() async {
    final Map<String, Object> values = await prefs.getAll();
    final int counter = (values['counter'] as int? ?? 0) + 1;

    setState(() {
      _counter = prefs.setValue('Int', 'counter', counter).then((bool success) {
        return counter;
      });
    });
  }

  @override
  void initState() {
    super.initState();
    _counter = prefs.getAll().then((Map<String, Object> values) {
      return values['counter'] as int? ?? 0;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SharedPreferences Demo'),
      ),
      body: Center(
          child: FutureBuilder<int>(
              future: _counter,
              builder: (BuildContext context, AsyncSnapshot<int> snapshot) {
                switch (snapshot.connectionState) {
                  case ConnectionState.none:
                  case ConnectionState.waiting:
                    return const CircularProgressIndicator();
                  case ConnectionState.active:
                  case ConnectionState.done:
                    if (snapshot.hasError) {
                      return Text('Error: ${snapshot.error}');
                    } else {
                      return Text(
                        'Button tapped ${snapshot.data} time${snapshot.data == 1 ? '' : 's'}.\n\n'
                        'This should persist across restarts.',
                      );
                    }
                }
              })),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:convert' show json;

import 'package:file/file.dart';
import 'package:file/local.dart';
import 'package:flutter/foundation.dart' show debugPrint, visibleForTesting;
import 'package:path/path.dart' as path;
import 'package:path_provider_linux/path_provider_linux.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_platform_interface.dart';

/// The Linux implementation of [SharedPreferencesStorePlatform].
///
/// This class implements the `package:shared_preferences` functionality for Linux.
class SharedPreferencesLinux extends SharedPreferencesStorePlatform {
  /// Deprecated instance of [SharedPreferencesLinux].
  /// Use [SharedPreferencesStorePlatform.instance] instead.
  @Deprecated('Use `SharedPreferencesStorePlatform.instance` instead.')
  static SharedPreferencesLinux instance = SharedPreferencesLinux();

  static const String _defaultPrefix = 'flutter.';

  /// Registers the Linux implementation.
  static void registerWith() {
    SharedPreferencesStorePlatform.instance = SharedPreferencesLinux();
  }

  /// Local copy of preferences
  Map<String, Object>? _cachedPreferences;

  /// File system used to store to disk. Exposed for testing only.
  @visibleForTesting
  FileSystem fs = const LocalFileSystem();

  /// The path_provider_linux instance used to find the support directory.
  @visibleForTesting
  PathProviderLinux pathProvider = PathProviderLinux();

  /// Gets the file where the preferences are stored.
  Future<File?> _getLocalDataFile() async {
    final String? directory = await pathProvider.getApplicationSupportPath();
    if (directory == null) {
      return null;
    }
    return fs.file(path.join(directory, 'shared_preferences.json'));
  }

  /// Gets the preferences from the stored file and saves them in cache.
  Future<Map<String, Object>> _reload() async {
    Map<String, Object> preferences = <String, Object>{};
    final File? localDataFile = await _getLocalDataFile();
    if (localDataFile != null && localDataFile.existsSync()) {
      final String stringMap = localDataFile.readAsStringSync();
      if (stringMap.isNotEmpty) {
        final Object? data = json.decode(stringMap);
        if (data is Map) {
          preferences = data.cast<String, Object>();
        }
      }
    }
    _cachedPreferences = preferences;
    return preferences;
  }

  /// Checks for cached preferences and returns them or loads preferences from
  /// file and returns and caches them.
  Future<Map<String, Object>> _readPreferences() async {
    return _cachedPreferences ?? await _reload();
  }

  /// Writes the cached preferences to disk. Returns [true] if the operation
  /// succeeded.
  Future<bool> _writePreferences(Map<String, Object> preferences) async {
    try {
      final File? localDataFile = await _getLocalDataFile();
      if (localDataFile == null) {
        debugPrint('Unable to determine where to write preferences.');
        return false;
      }
      if (!localDataFile.existsSync()) {
        localDataFile.createSync(recursive: true);
      }
      final String stringMap = json.encode(preferences);
      localDataFile.writeAsStringSync(stringMap);
    } catch (e) {
      debugPrint('Error saving preferences to disk: $e');
      return false;
    }
    return true;
  }

  @override
  Future<bool> clear() async {
    return clearWithPrefix(_defaultPrefix);
  }

  @override
  Future<bool> clearWithPrefix(String prefix) async {
    final Map<String, Object> preferences = await _readPreferences();
    preferences.removeWhere((String key, _) => key.startsWith(prefix));
    return _writePreferences(preferences);
  }

  @override
  Future<Map<String, Object>> getAll() async {
    return getAllWithPrefix(_defaultPrefix);
  }

  @override
  Future<Map<String, Object>> getAllWithPrefix(String prefix) async {
    final Map<String, Object> withPrefix =
        Map<String, Object>.from(await _readPreferences());
    withPrefix.removeWhere((String key, _) => !key.startsWith(prefix));
    return withPrefix;
  }

  @override
  Future<bool> remove(String key) async {
    final Map<String, Object> preferences = await _readPreferences();
    preferences.remove(key);
    return _writePreferences(preferences);
  }

  @override
  Future<bool> setValue(String valueType, String key, Object value) async {
    final Map<String, Object> preferences = await _readPreferences();
    preferences[key] = value;
    return _writePreferences(preferences);
  }
}
```

```dart
import 'package:device_info_plus_linux/device_info_plus_linux.dart';
import 'package:device_info_plus_platform_interface/device_info_plus_platform_interface.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:file/memory.dart';

void main() {
  test('registered instance', () {
    DeviceInfoLinux.registerWith();
    expect(DeviceInfoPlatform.instance, isA<DeviceInfoLinux>());
  });
  test('os-release', () async {
    final fs = MemoryFileSystem.test();
    final file = fs.file('/etc/os-release')..createSync(recursive: true);
    file.writeAsStringSync('''
NAME="A Linux"
VERSION="1.2.3 LTS (A Linux)"
ID=foo
ID_LIKE="bar baz"
VERSION_CODENAME=lts
VERSION_ID="1.2.3-lts"
PRETTY_NAME="A Linux 1.2.3 LTS"
BUILD_ID=1
VARIANT="Community Edition"
VARIANT_ID=community
HOME_URL="https://www.fluttercommunity.dev/"
    ''');

    final deviceInfo = DeviceInfoLinux(fileSystem: fs);
    final linuxInfo = await deviceInfo.linuxInfo();
    expect(linuxInfo.name, equals('A Linux'));
    expect(linuxInfo.version, equals('1.2.3 LTS (A Linux)'));
    expect(linuxInfo.id, equals('foo'));
    expect(linuxInfo.idLike, equals(['bar', 'baz']));
    expect(linuxInfo.versionCodename, equals('lts'));
    expect(linuxInfo.versionId, equals('1.2.3-lts'));
    expect(linuxInfo.prettyName, equals('A Linux 1.2.3 LTS'));
    expect(linuxInfo.buildId, equals('1'));
    expect(linuxInfo.variant, equals('Community Edition'));
    expect(linuxInfo.variantId, equals('community'));
  });

  test('lsb-release', () async {
    final fs = MemoryFileSystem.test();
    final file = fs.file('/etc/lsb-release')..createSync(recursive: true);
    file.writeAsStringSync('''
LSB_VERSION="LSB version"
DISTRIB_ID=distrib-id
DISTRIB_RELEASE=distrib-release
DISTRIB_CODENAME=distrib-codename
DISTRIB_DESCRIPTION="Distrib Description"
    ''');

    final deviceInfo = DeviceInfoLinux(fileSystem: fs);
    final linuxInfo = await deviceInfo.linuxInfo();
    expect(linuxInfo.name, equals('Linux'));
    expect(linuxInfo.version, equals('LSB version'));
    expect(linuxInfo.id, equals('distrib-id'));
    expect(linuxInfo.idLike, isNull);
    expect(linuxInfo.versionCodename, equals('distrib-codename'));
    expect(linuxInfo.versionId, equals('distrib-release'));
    expect(linuxInfo.prettyName, 'Distrib Description');
    expect(linuxInfo.buildId, isNull);
    expect(linuxInfo.variant, isNull);
    expect(linuxInfo.variantId, isNull);
  });

  test('precedence', () async {
    final fs = MemoryFileSystem.test();
    final osFile = fs.file('/etc/os-release')..createSync(recursive: true);
    osFile.writeAsStringSync('''
VERSION="OS version"
ID=os
    ''');
    final lsbFile = fs.file('/etc/lsb-release')..createSync(recursive: true);
    lsbFile.writeAsStringSync('''
LSB_VERSION="LSB version"
DISTRIB_ID=lsb
DISTRIB_RELEASE=distrib-release
DISTRIB_CODENAME=distrib-codename
DISTRIB_DESCRIPTION="Distrib Description"
    ''');

    final deviceInfo = DeviceInfoLinux(fileSystem: fs);
    final linuxInfo = await deviceInfo.linuxInfo();
    expect(linuxInfo.name, equals('Linux'));
    expect(linuxInfo.version, equals('OS version'));
    expect(linuxInfo.id, equals('os'));
    expect(linuxInfo.idLike, isNull);
    expect(linuxInfo.versionCodename, equals('distrib-codename'));
    expect(linuxInfo.versionId, equals('distrib-release'));
    expect(linuxInfo.prettyName, 'Distrib Description');
    expect(linuxInfo.buildId, isNull);
    expect(linuxInfo.variant, isNull);
    expect(linuxInfo.variantId, isNull);
  });

  test('machine-id', () async {
    final fs = MemoryFileSystem.test();
    final file = fs.file('/etc/machine-id')..createSync(recursive: true);
    file.writeAsStringSync('machine-id');

    final deviceInfo = DeviceInfoLinux(fileSystem: fs);
    final linuxInfo = await deviceInfo.linuxInfo();
    expect(linuxInfo.machineId, equals('machine-id'));
  });

  test('missing files', () async {
    final fs = MemoryFileSystem.test();
    final deviceInfo = DeviceInfoLinux(fileSystem: fs);
    final linuxInfo = await deviceInfo.linuxInfo();
    expect(linuxInfo.name, equals('Linux'));
    expect(linuxInfo.version, isNull);
    expect(linuxInfo.id, equals('linux'));
    expect(linuxInfo.idLike, isNull);
    expect(linuxInfo.versionCodename, isNull);
    expect(linuxInfo.versionId, isNull);
    expect(linuxInfo.prettyName, 'Linux');
    expect(linuxInfo.buildId, isNull);
    expect(linuxInfo.variant, isNull);
    expect(linuxInfo.variantId, isNull);
    expect(linuxInfo.machineId, isNull);
  });
}
```

```dart
/// The Linux implementation of `device_info_plus`.
library device_info_plus_linux;

export 'src/device_info.dart';
```

```dart
import 'dart:async';

import 'package:device_info_plus_platform_interface/device_info_plus_platform_interface.dart';
import 'package:file/file.dart';
import 'package:file/local.dart';
import 'package:meta/meta.dart';

/// See [DeviceInfoPlatform]
class DeviceInfoLinux extends DeviceInfoPlatform {
  /// Register this dart class as the platform implementation for linux
  static void registerWith() {
    DeviceInfoPlatform.instance = DeviceInfoLinux();
  }

  LinuxDeviceInfo? _cache;
  final FileSystem _fileSystem;

  ///
  DeviceInfoLinux({@visibleForTesting FileSystem? fileSystem})
      : _fileSystem = fileSystem ?? const LocalFileSystem();

  @override
  Future<LinuxDeviceInfo> linuxInfo() async {
    return _cache ??= await _getInfo();
  }

  Future<LinuxDeviceInfo> _getInfo() async {
    final os = await _getOsRelease() ?? {};
    final lsb = await _getLsbRelease() ?? {};
    final machineId = await _getMachineId();

    return LinuxDeviceInfo(
      name: os['NAME'] ?? 'Linux',
      version: os['VERSION'] ?? lsb['LSB_VERSION'],
      id: os['ID'] ?? lsb['DISTRIB_ID'] ?? 'linux',
      idLike: os['ID_LIKE']?.split(' '),
      versionCodename: os['VERSION_CODENAME'] ?? lsb['DISTRIB_CODENAME'],
      versionId: os['VERSION_ID'] ?? lsb['DISTRIB_RELEASE'],
      prettyName: os['PRETTY_NAME'] ?? lsb['DISTRIB_DESCRIPTION'] ?? 'Linux',
      buildId: os['BUILD_ID'],
      variant: os['VARIANT'],
      variantId: os['VARIANT_ID'],
      machineId: machineId,
    );
  }

  Future<Map<String, String?>?> _getOsRelease() {
    return _tryReadKeyValues('/etc/os-release').then((value) async =>
        value ?? await _tryReadKeyValues('/usr/lib/os-release'));
  }

  Future<Map<String, String?>?> _getLsbRelease() {
    return _tryReadKeyValues('/etc/lsb-release');
  }

  Future<String?> _getMachineId() {
    return _tryReadValue('/etc/machine-id');
  }

  Future<String?> _tryReadValue(String path) {
    return _fileSystem
        .file(path)
        .readAsString()
        .then((str) => str.trim(), onError: (_) => null);
  }

  Future<Map<String, String?>?> _tryReadKeyValues(String path) {
    return _fileSystem
        .file(path)
        .readAsLines()
        .then((lines) => lines.toKeyValues(), onError: (_) => null);
  }
}

extension _Unquote on String {
  String removePrefix(String prefix) {
    if (!startsWith(prefix)) return this;
    return substring(prefix.length);
  }

  String removeSuffix(String suffix) {
    if (!endsWith(suffix)) return this;
    return substring(0, length - suffix.length);
  }

  String unquote() {
    return removePrefix('"').removeSuffix('"');
  }
}

extension _KeyValues on List<String> {
  Map<String, String?> toKeyValues() {
    return Map.fromEntries(map((line) {
      final parts = line.split('=');
      if (parts.length != 2) return MapEntry(line, null);
      return MapEntry(parts.first, parts.last.unquote());
    }));
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
import 'package:flutter_test/flutter_test.dart';
import 'package:path_provider_linux/path_provider_linux.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:xdg_directories/xdg_directories.dart' as xdg;

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  PathProviderLinux.registerWith();

  test('registered instance', () {
    expect(PathProviderPlatform.instance, isA<PathProviderLinux>());
  });

  test('getTemporaryPath defaults to TMPDIR', () async {
    final PathProviderPlatform plugin = PathProviderLinux.private(
      environment: <String, String>{'TMPDIR': '/run/user/0/tmp'},
    );
    expect(await plugin.getTemporaryPath(), '/run/user/0/tmp');
  });

  test('getTemporaryPath uses fallback if TMPDIR is empty', () async {
    final PathProviderPlatform plugin = PathProviderLinux.private(
      environment: <String, String>{'TMPDIR': ''},
    );
    expect(await plugin.getTemporaryPath(), '/tmp');
  });

  test('getTemporaryPath uses fallback if TMPDIR is unset', () async {
    final PathProviderPlatform plugin = PathProviderLinux.private(
      environment: <String, String>{},
    );
    expect(await plugin.getTemporaryPath(), '/tmp');
  });

  test('getApplicationSupportPath', () async {
    final PathProviderPlatform plugin = PathProviderLinux.private(
        executableName: 'path_provider_linux_test_binary',
        applicationId: 'com.example.Test');
    // Note this will fail if ${xdg.dataHome.path}/path_provider_linux_test_binary exists on the local filesystem.
    expect(await plugin.getApplicationSupportPath(),
        '${xdg.dataHome.path}/com.example.Test');
  });

  test('getApplicationSupportPath uses executable name if no application Id',
      () async {
    final PathProviderPlatform plugin = PathProviderLinux.private(
        executableName: 'path_provider_linux_test_binary');
    expect(await plugin.getApplicationSupportPath(),
        '${xdg.dataHome.path}/path_provider_linux_test_binary');
  });

  test('getApplicationDocumentsPath', () async {
    final PathProviderPlatform plugin = PathProviderPlatform.instance;
    expect(await plugin.getApplicationDocumentsPath(), startsWith('/'));
  });

  test('getDownloadsPath', () async {
    final PathProviderPlatform plugin = PathProviderPlatform.instance;
    expect(await plugin.getDownloadsPath(), startsWith('/'));
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path_provider_linux/src/get_application_id_real.dart';

class _FakeGioUtils implements GioUtils {
  int? application;
  Pointer<Utf8>? applicationId;

  @override
  bool libraryIsPresent = false;

  @override
  int gApplicationGetDefault() => application!;

  @override
  Pointer<Utf8> gApplicationGetApplicationId(int app) => applicationId!;
}

void main() {
  late _FakeGioUtils fakeGio;

  setUp(() {
    fakeGio = _FakeGioUtils();
    gioUtilsOverride = fakeGio;
  });

  tearDown(() {
    gioUtilsOverride = null;
  });

  test('returns null if libgio is not available', () {
    expect(getApplicationId(), null);
  });

  test('returns null if g_paplication_get_default returns 0', () {
    fakeGio.libraryIsPresent = true;
    fakeGio.application = 0;
    expect(getApplicationId(), null);
  });

  test('returns null if g_application_get_application_id returns nullptr', () {
    fakeGio.libraryIsPresent = true;
    fakeGio.application = 1;
    fakeGio.applicationId = nullptr;
    expect(getApplicationId(), null);
  });

  test('returns value if g_application_get_application_id returns a value', () {
    fakeGio.libraryIsPresent = true;
    fakeGio.application = 1;
    const String id = 'foo';
    final Pointer<Utf8> idPtr = id.toNativeUtf8();
    fakeGio.applicationId = idPtr;
    expect(getApplicationId(), id);
    calloc.free(idPtr);
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider_linux/path_provider_linux.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('getTemporaryDirectory', (WidgetTester tester) async {
    final PathProviderLinux provider = PathProviderLinux();
    final String? result = await provider.getTemporaryPath();
    _verifySampleFile(result, 'temporaryDirectory');
  });

  testWidgets('getDownloadDirectory', (WidgetTester tester) async {
    if (!Platform.isLinux) {
      return;
    }
    final PathProviderLinux provider = PathProviderLinux();
    final String? result = await provider.getDownloadsPath();
    _verifySampleFile(result, 'downloadDirectory');
  });

  testWidgets('getApplicationDocumentsDirectory', (WidgetTester tester) async {
    final PathProviderLinux provider = PathProviderLinux();
    final String? result = await provider.getApplicationDocumentsPath();
    _verifySampleFile(result, 'applicationDocuments');
  });

  testWidgets('getApplicationSupportDirectory', (WidgetTester tester) async {
    final PathProviderLinux provider = PathProviderLinux();
    final String? result = await provider.getApplicationSupportPath();
    _verifySampleFile(result, 'applicationSupport');
  });
}

/// Verify a file called [name] in [directoryPath] by recreating it with test
/// contents when necessary.
void _verifySampleFile(String? directoryPath, String name) {
  expect(directoryPath, isNotNull);
  if (directoryPath == null) {
    return;
  }
  final Directory directory = Directory(directoryPath);
  final File file = File('${directory.path}${Platform.pathSeparator}$name');

  if (file.existsSync()) {
    file.deleteSync();
    expect(file.existsSync(), isFalse);
  }

  file.writeAsStringSync('Hello world!');
  expect(file.readAsStringSync(), 'Hello world!');
  expect(directory.listSync(), isNotEmpty);
  file.deleteSync();
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path_provider_linux/path_provider_linux.dart';

void main() {
  runApp(const MyApp());
}

/// Sample app
class MyApp extends StatefulWidget {
  /// Default Constructor
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String? _tempDirectory = 'Unknown';
  String? _downloadsDirectory = 'Unknown';
  String? _appSupportDirectory = 'Unknown';
  String? _documentsDirectory = 'Unknown';
  final PathProviderLinux _provider = PathProviderLinux();

  @override
  void initState() {
    super.initState();
    initDirectories();
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initDirectories() async {
    String? tempDirectory;
    String? downloadsDirectory;
    String? appSupportDirectory;
    String? documentsDirectory;
    // Platform messages may fail, so we use a try/catch PlatformException.
    try {
      tempDirectory = await _provider.getTemporaryPath();
    } on PlatformException {
      tempDirectory = 'Failed to get temp directory.';
    }
    try {
      downloadsDirectory = await _provider.getDownloadsPath();
    } on PlatformException {
      downloadsDirectory = 'Failed to get downloads directory.';
    }

    try {
      documentsDirectory = await _provider.getApplicationDocumentsPath();
    } on PlatformException {
      documentsDirectory = 'Failed to get documents directory.';
    }

    try {
      appSupportDirectory = await _provider.getApplicationSupportPath();
    } on PlatformException {
      appSupportDirectory = 'Failed to get documents directory.';
    }
    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) {
      return;
    }

    setState(() {
      _tempDirectory = tempDirectory;
      _downloadsDirectory = downloadsDirectory;
      _appSupportDirectory = appSupportDirectory;
      _documentsDirectory = documentsDirectory;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Path Provider Linux example app'),
        ),
        body: Center(
          child: Column(
            children: <Widget>[
              Text('Temp Directory: $_tempDirectory\n'),
              Text('Documents Directory: $_documentsDirectory\n'),
              Text('Downloads Directory: $_downloadsDirectory\n'),
              Text('Application Support Directory: $_appSupportDirectory\n'),
            ],
          ),
        ),
      ),
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

export 'src/path_provider_linux.dart';
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:xdg_directories/xdg_directories.dart' as xdg;

import 'get_application_id.dart';

/// The linux implementation of [PathProviderPlatform]
///
/// This class implements the `package:path_provider` functionality for Linux.
class PathProviderLinux extends PathProviderPlatform {
  /// Constructs an instance of [PathProviderLinux]
  PathProviderLinux() : _environment = Platform.environment;

  /// Constructs an instance of [PathProviderLinux] with the given [environment]
  @visibleForTesting
  PathProviderLinux.private(
      {Map<String, String> environment = const <String, String>{},
      String? executableName,
      String? applicationId})
      : _environment = environment,
        _executableName = executableName,
        _applicationId = applicationId;

  final Map<String, String> _environment;
  String? _executableName;
  String? _applicationId;

  /// Registers this class as the default instance of [PathProviderPlatform]
  static void registerWith() {
    PathProviderPlatform.instance = PathProviderLinux();
  }

  @override
  Future<String?> getTemporaryPath() {
    final String environmentTmpDir = _environment['TMPDIR'] ?? '';
    return Future<String?>.value(
      environmentTmpDir.isEmpty ? '/tmp' : environmentTmpDir,
    );
  }

  @override
  Future<String?> getApplicationSupportPath() async {
    final Directory directory =
        Directory(path.join(xdg.dataHome.path, await _getId()));
    if (directory.existsSync()) {
      return directory.path;
    }

    // This plugin originally used the executable name as a directory.
    // Use that if it exists for backwards compatibility.
    final Directory legacyDirectory =
        Directory(path.join(xdg.dataHome.path, await _getExecutableName()));
    if (legacyDirectory.existsSync()) {
      return legacyDirectory.path;
    }

    // Create the directory, because mobile implementations assume the directory exists.
    await directory.create(recursive: true);
    return directory.path;
  }

  @override
  Future<String?> getApplicationDocumentsPath() {
    return Future<String?>.value(xdg.getUserDirectory('DOCUMENTS')?.path);
  }

  @override
  Future<String?> getDownloadsPath() {
    return Future<String?>.value(xdg.getUserDirectory('DOWNLOAD')?.path);
  }

  // Gets the name of this executable.
  Future<String> _getExecutableName() async {
    _executableName ??= path.basenameWithoutExtension(
        await File('/proc/self/exe').resolveSymbolicLinks());
    return _executableName!;
  }

  // Gets the unique ID for this application.
  Future<String> _getId() async {
    _applicationId ??= getApplicationId();
    // If no application ID then fall back to using the executable name.
    return _applicationId ?? await _getExecutableName();
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// getApplicationId() is implemented using FFI; export a stub for platforms
// that don't support FFI (e.g., web) to avoid having transitive dependencies
// break web compilation.
export 'get_application_id_stub.dart'
    if (dart.library.ffi) 'get_application_id_real.dart';
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Gets the application ID for this app.
String? getApplicationId() => null;
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart' show visibleForTesting;

// GApplication* g_application_get_default();
typedef _GApplicationGetDefaultC = IntPtr Function();
typedef _GApplicationGetDefaultDart = int Function();

// const gchar* g_application_get_application_id(GApplication* application);
typedef _GApplicationGetApplicationIdC = Pointer<Utf8> Function(IntPtr);
typedef _GApplicationGetApplicationIdDart = Pointer<Utf8> Function(int);

/// Interface for interacting with libgio.
@visibleForTesting
class GioUtils {
  /// Creates a default instance that uses the real libgio.
  GioUtils() {
    try {
      _gio = DynamicLibrary.open('libgio-2.0.so');
    } on ArgumentError {
      _gio = null;
    }
  }

  DynamicLibrary? _gio;

  /// True if libgio was opened successfully.
  bool get libraryIsPresent => _gio != null;

  /// Wraps `g_application_get_default`.
  int gApplicationGetDefault() {
    if (_gio == null) {
      return 0;
    }
    final _GApplicationGetDefaultDart getDefault = _gio!
        .lookupFunction<_GApplicationGetDefaultC, _GApplicationGetDefaultDart>(
            'g_application_get_default');
    return getDefault();
  }

  /// Wraps g_application_get_application_id.
  Pointer<Utf8> gApplicationGetApplicationId(int app) {
    if (_gio == null) {
      return nullptr;
    }
    final _GApplicationGetApplicationIdDart gApplicationGetApplicationId = _gio!
        .lookupFunction<_GApplicationGetApplicationIdC,
                _GApplicationGetApplicationIdDart>(
            'g_application_get_application_id');
    return gApplicationGetApplicationId(app);
  }
}

/// Allows overriding the default GioUtils instance with a fake for testing.
@visibleForTesting
GioUtils? gioUtilsOverride;

/// Gets the application ID for this app.
String? getApplicationId() {
  final GioUtils gio = gioUtilsOverride ?? GioUtils();
  if (!gio.libraryIsPresent) {
    return null;
  }

  final int app = gio.gApplicationGetDefault();
  if (app == 0) {
    return null;
  }
  final Pointer<Utf8> appId = gio.gApplicationGetApplicationId(app);
  if (appId == null || appId == nullptr) {
    return null;
  }
  return appId.toDartString();
}
```

```dart
// DO NOT EDIT. This is code generated via package:get_cli/get_cli.dart

// ignore_for_file: lines_longer_than_80_chars
// ignore: avoid_classes_with_only_static_members
class AppTranslation {
  static Map<String, Map<String, String>> translations = {
    'zh_CN': Locales.zh_CN,
    'en_US': Locales.en_US,
  };
}

class LocaleKeys {
  LocaleKeys._();
  static const buttons_login = 'buttons_login';
  static const buttons_sign_in = 'buttons_sign_in';
  static const buttons_logout = 'buttons_logout';
  static const buttons_sign_in_fb = 'buttons_sign_in_fb';
  static const buttons_sign_in_google = 'buttons_sign_in_google';
  static const buttons_sign_in_apple = 'buttons_sign_in_apple';
  static const nav_home = 'nav_home';
  static const nav_settings = 'nav_settings';
  static const menu_language = 'menu_language';
  static const menu_privacy = 'menu_privacy';
  static const menu_terms = 'menu_terms';
  static const menu_about = 'menu_about';
  static const appbar_setting = 'appbar_setting';
}

class Locales {
  static const zh_CN = {
    'buttons_login': '登录',
    'buttons_sign_in': '注册',
    'buttons_logout': '注销',
    'buttons_sign_in_fb': '用 Facebook 登录',
    'buttons_sign_in_google': '用 Google 登录',
    'buttons_sign_in_apple': '用 Apple 登录',
    'nav_home': '首页',
    'nav_settings': '设置',
    'menu_language': '语言设置',
    'menu_privacy': '隐私政策',
    'menu_terms': '隐私条款',
    'menu_about': '隐私条款',
    'appbar_setting': '用户设置',
  };
  static const en_US = {
    'buttons_login': 'Login',
    'buttons_sign_in': 'Sign-in',
    'buttons_logout': 'Logout',
    'buttons_sign_in_fb': 'Sign-in with Facebook',
    'buttons_sign_in_google': 'Sign-in with Google',
    'buttons_sign_in_apple': 'Sign-in with Apple',
    'nav_home': 'Home',
    'nav_settings': 'Settings',
    'menu_language': 'Language',
    'menu_privacy': 'Privacy Policy',
    'menu_terms': 'Terms Of Service',
    'menu_about': 'About SleepEase',
    'appbar_setting': 'Settings',
  };
}
```

```dart
import 'package:sleepease/src/common/config/sleep_global_config.dart';
import 'package:wheel/wheel.dart';

void main() {
  CommonUtils.initialApp(ConfigType.PRO).whenComplete(() => {SleepGlobalConfig.loadApp(ConfigType.PRO)});
}```

```dart
import 'dart:io';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:get/get.dart';

class PrivacyController extends GetxController {
  var privacyPolicy = "".obs;

  @override
  void onInit() {
    super.onInit();
    readPrivacy();
  }
  Future<void> readPrivacy() async {
    String filePath = 'assets/law/privacy.md';
    String fileData = await rootBundle.loadString(filePath);
    privacyPolicy.value = fileData;
  }

  TextStyle textStyle = TextStyle(color: Colors.black, fontSize: 18);
}
```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:get/get.dart';
import 'package:sleepease/src/page/settings/setting/privacy/privacy_controller.dart';

class Privacy extends StatelessWidget {

  @override
  Widget build(BuildContext context) {
    return GetBuilder<PrivacyController>(
        init: PrivacyController(),
        builder: (controller) {
          return Scaffold(
              body: SafeArea(
                child:SingleChildScrollView(
              child:Padding(
                  padding: EdgeInsets.all(16.0),
                  child:Obx(()=>MarkdownBody(data: controller.privacyPolicy.value))
              ))));
        }
    );
  }
}```

```dart
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:get/get.dart';

class TermsOfServiceController extends GetxController {
  var termsOfService = "".obs;

  @override
  void onInit() {
    super.onInit();
    readTermsOfService();
  }
  Future<void> readTermsOfService() async {
    String filePath = 'assets/law/terms-of-service.md';
    String fileData = await rootBundle.loadString(filePath);
    termsOfService.value = fileData;
  }

  TextStyle textStyle = TextStyle(color: Colors.black, fontSize: 18);
}```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:get/get.dart';

import 'terms_of_service_controller.dart';

class TermsOfService extends StatelessWidget {

  TextStyle textStyle = TextStyle(
    color: Colors.black,
    fontSize: 18
  );

  @override
  Widget build(BuildContext context) {
    return GetBuilder<TermsOfServiceController>(
        init: TermsOfServiceController(),
        builder: (controller) {
          return Scaffold(
              body: SafeArea(
                  child:SingleChildScrollView(
                      child:Padding(
                          padding: EdgeInsets.all(16.0),
                          child:Obx(()=>MarkdownBody(data: controller.termsOfService.value))
                      ))));
        }
    );
  }
}```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import '../setting_controller.dart';
import 'language_controller.dart';

class Language extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return GetBuilder<LanguageController>(
        init: LanguageController(),
        builder: (controller) {
          final settingController = Get.find<SettingController>();
          void _changeLanguage(Locale locale) {
            settingController.updateLocale(locale);
          }

          return Scaffold(
              backgroundColor: Color.fromRGBO(235, 233, 241, 1),
              body: SafeArea(
                  child: Container(
                      padding: EdgeInsets.all(20),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: <Widget>[
                          Card(
                            elevation: 3,
                            shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(10),
                            ),
                            child: Column(
                              children: [
                                ListTile(
                                  title: Text("English"),
                                  onTap: () => _changeLanguage(const Locale('en', 'US')),
                                  trailing: Checkbox(
                                    value: Get.locale == Locale('en', 'US'),
                                    shape: CircleBorder(),
                                    onChanged: (bool? value) {
                                      settingController.updateLocale(Locale('en', 'US'));
                                    },
                                  ),
                                ),
                                Divider(height: 2,thickness:1),
                                ListTile(
                                  title: Text('中文'),
                                  onTap: () => _changeLanguage(const Locale('zh', 'CN')),
                                  trailing: Checkbox(
                                    value: Get.locale  == Locale('zh', 'CN'),
                                    shape: CircleBorder(),
                                    onChanged: (bool? value) {
                                      settingController.updateLocale(Locale('zh', 'CN'));
                                    },
                                  ),
                                ),
                                Divider(height: 2,thickness:1),
                              ],
                            ),
                          ),
                        ],
                      ))));
        });
  }
}
```

```dart
import 'dart:ui';

import 'package:get/get.dart';

class LanguageController extends GetxController {
}```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'about_controller.dart';

class About extends StatelessWidget {

  TextStyle textStyle = TextStyle(
    color: Colors.black,
    fontSize: 16
  );

  @override
  Widget build(BuildContext context) {
    return GetBuilder<AboutController>(
        init: AboutController(),
        builder: (controller) {
          return Scaffold(
              body: SafeArea(child: Padding(
              padding: EdgeInsets.all(16.0),
          child:Text("SleepEase是一款专注于帮助人们入睡的应用，通过播放大自然的声音，如海浪声、深林鸟鸣声、雨声等，为用户打造出一个轻松舒适的睡眠环境。无论您是因为工作压力大、焦虑情绪等原因导致失眠，还是长期睡眠不足，SleepEase都能够帮助您调节睡眠，让您在音乐的陪伴下渐渐入睡。同时，SleepEase拥有多种音效可供选择，你可以根据自己的喜好自由切换，定时开启，让您能够充分休息，并迎接健康的新一天。快来下载并使用SleepEase，让它成为您入睡前必不可少的助手吧！",
              style: textStyle,),)));
        }
    );
  }
}```

```dart
import 'package:get/get.dart';

class AboutController extends GetxController {

}```

```dart
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:get_storage/get_storage.dart';

class SettingController extends GetxController {
  var themeMode = ThemeMode.system.obs;

  @override
  void onInit() {
    super.onInit();
    var box = GetStorage();
    if (box.hasData('themeMode')) {
      themeMode.value = ThemeMode.values[box.read('themeMode')];
    }
    if(box.hasData("language")){
      String local = box.read("locale");
      Get.updateLocale(localeFromJson(local));
    }
  }

  String localeToJson(Locale locale) {
    return json.encode({
      'languageCode': locale.languageCode,
      'countryCode': locale.countryCode,
    });
  }

  Locale localeFromJson(String jsonString) {
    Map<String, dynamic> jsonMap = json.decode(jsonString);
    return Locale(jsonMap['languageCode'], jsonMap['countryCode']);
  }

  void updateLocale(Locale newLocale){
    Get.updateLocale(newLocale);
    var box = GetStorage();
    box.write('locale', localeToJson(newLocale));
    Get.forceAppUpdate();
  }

  void updateThemeMode(ThemeMode mode) {
    themeMode.value = mode;
    var box = GetStorage();
    box.write('themeMode', mode.index);
  }
}```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:sleepease/src/page/settings/setting/privacy/privacy.dart';
import 'package:sleepease/src/page/settings/setting/setting_controller.dart';
import 'package:sleepease/src/page/settings/setting/terms/terms_of_service.dart';
import '../../../../generated/locales.g.dart';
import 'about/about.dart';
import 'language/language.dart';

class Setting extends StatelessWidget {
  TextStyle textStyle = TextStyle(
    color: Colors.black,
  );

  @override
  Widget build(BuildContext context) {
    return GetBuilder<SettingController>(
        init: SettingController(),
        builder: (controller) {
          return Scaffold(
              backgroundColor: Color.fromRGBO(235, 233, 241, 1),
              appBar: AppBar(
                backgroundColor: Color.fromRGBO(235, 233, 241, 1),
                title: Text(
                  LocaleKeys.appbar_setting.tr,
                  style: textStyle,
                ),
              ),
              body: SafeArea(
                  child: Container(
                padding: EdgeInsets.all(20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Card(
                      elevation: 3,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Column(
                        children: [],
                      ),
                    ),
                    SizedBox(height: 20),
                    Card(
                      elevation: 3,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Column(
                        children: [
                          ListTile(
                            leading: Icon(
                              Icons.language,
                              color: Colors.teal,
                            ),
                            title: Text(LocaleKeys.menu_language.tr),
                            trailing: Icon(Icons.arrow_forward_ios_rounded),
                            onTap: ()=>{Get.to(Language())},
                          ),
                        ],
                      ),
                    ),
                    SizedBox(height: 20),
                    Card(
                      elevation: 3,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Column(
                        children: [
                          ListTile(
                            leading: Icon(
                              Icons.privacy_tip,
                              color: Colors.pinkAccent,
                            ),
                            title: Text(LocaleKeys.menu_privacy.tr),
                            trailing: Icon(Icons.arrow_forward_ios_rounded),
                            onTap: () => {Get.to(Privacy())},
                          ),
                          Divider(),
                          ListTile(
                            leading: Icon(
                              Icons.abc,
                              color: Colors.green,
                            ),
                            title: Text(LocaleKeys.menu_terms.tr),
                            trailing: Icon(Icons.arrow_forward_ios_rounded),
                            onTap: () => {Get.to(TermsOfService())},
                          ),
                          Divider(),
                          ListTile(
                            leading: Icon(
                              Icons.adb,
                              color: Colors.teal,
                            ),
                            title: Text(LocaleKeys.menu_about.tr),
                            trailing: Icon(Icons.arrow_forward_ios_rounded),
                            onTap: () => {Get.to(About())},
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              )));
        });
  }
}
```

```dart
import 'package:audioplayers/audioplayers.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'home_controller.dart';

class HomeDefault extends StatelessWidget {
  @override
  Widget build(BuildContext context) {

    return GetBuilder<HomeController>(
        init: HomeController(),
        builder: (controller) {
          return SafeArea(
              child: Column(
            children: [
              Expanded(
                child: PageView.builder(
                    itemCount: controller.pages.length,
                    onPageChanged: (int page) {
                      controller.currentPage.value = page;
                      String audioPath = controller.pages[controller.currentPage.value]['audio'];
                      controller.audioPlayer.setSource(AssetSource(audioPath));
                    },
                    itemBuilder: (BuildContext context, int index) {
                      return Image.asset(
                        controller.pages[index]['image'],
                        fit: BoxFit.cover,
                      );
                    }),
              ),
              Obx(() => LinearProgressIndicator(
                value: controller.playProgress.value,
                semanticsLabel: 'Linear progress indicator',
              )),
              Container(
                padding: EdgeInsets.symmetric(vertical: 10.0, horizontal: 20.0),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Obx(()=>Text("${controller.currentPage.value + 1}/${controller.pages.length}")),
                    IconButton(
                      icon: Obx(()=>Icon(controller.isPlaying.value ? Icons.pause : Icons.play_arrow)),
                      onPressed: () {
                        String audioPath = controller.pages[controller.currentPage.value]['audio'];
                        controller.audioPlayer.setSource(AssetSource(audioPath));
                        if (controller.isPlaying.value) {
                          controller.audioPlayer.pause();
                        } else {
                          controller.audioPlayer.resume();
                        }
                        controller.isPlaying.value = !controller.isPlaying.value;
                      },
                    )
                  ],
                ),
              )
            ],
          ));
        });
  }
}
```

```dart
import 'package:audioplayers/audioplayers.dart';
import 'package:get/get.dart';

class HomeController extends GetxController {
  var currentPage = 0.obs;
  var isPlaying = false.obs;
  RxDouble playProgress = 0.0.obs;
  final audioPlayer = AudioPlayer();
  List<Map<String, dynamic>> pages = [
    {
      'image': 'assets/images/780.jpg',
      'audio': 'audio/birds.mp3',
    },
    {
      'image': 'assets/images/OIP-C.jpeg',
      'audio': 'audio/sample-15s.mp3',
    },
    {
      'image': 'assets/images/233309.jpg',
      'audio': 'audio/mixkit-tech-house-vibes-130.mp3',
    },
  ];

  @override
  void onInit() {
    super.onInit();
    _updateProgress(audioPlayer);
    handlePlayCompletion();
    handlePlayChanged();
  }

  void handlePlayChanged(){
    audioPlayer.onPlayerStateChanged.listen((state) {
      if (state == PlayerState.completed) {
        if(currentPage.value==pages.length -1){
          audioPlayer.stop();
          isPlaying.value = false;
        }else{
          currentPage.value = currentPage.value + 1;
          _playSong(pages[currentPage.value]["audio"]);
        }
      }
    });
  }

  void _playSong(String url) async {
    await audioPlayer.play(AssetSource(url));
  }

  void handlePlayCompletion(){
    audioPlayer.onPlayerComplete.listen((event) {

    });
  }

  void _updateProgress(AudioPlayer _audioPlayer) async {
    while (true) {
      Duration? currentPosition = await _audioPlayer.getCurrentPosition();
      Duration? totalPosition = await _audioPlayer.getDuration();
      if (currentPosition != null && totalPosition != null) {
        double progress = currentPosition.inSeconds / totalPosition.inSeconds;
        playProgress.value = progress;
        await Future.delayed(Duration(seconds: 1));
      }
    }
  }
}
```

```dart
import 'package:get/get.dart';

class NavController extends GetxController {
  var selectIndex = 0.obs;
  var autoTriggerNav = false.obs;

  void updateNav(int index) {
    selectIndex.value = index;
    update();
  }
}
```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:sleepease/generated/locales.g.dart';
import 'package:sleepease/src/page/home/home.dart';
import 'package:sleepease/src/page/settings/setting/setting.dart';
import 'nav_controller.dart';

class Nav extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return GetBuilder<NavController>(
        init: NavController(),
        builder: (controller) {
          final locale = Localizations.localeOf(context);
          final locale1  = Get.locale;
          if (controller.autoTriggerNav.value) {}

          void _onItemTapped(int index) {
            controller.updateNav(index);
          }

          Widget currentMenu() {
            if (controller.selectIndex.value == 0) {
              return HomeDefault();
            }
            if (controller.selectIndex.value == 1) {
              return Setting();
            }
            return Text("开发中，敬请期待...");
          }

          return Scaffold(
            body: currentMenu(),
            bottomNavigationBar: BottomNavigationBar(
                items: [
                  BottomNavigationBarItem(
                      icon: GestureDetector(onDoubleTap: () {}, child: Icon(Icons.home)), label: LocaleKeys.nav_home.tr),
                  BottomNavigationBarItem(icon: Icon(Icons.settings), label: LocaleKeys.buttons_login.tr),
                ],
                currentIndex: controller.selectIndex.value,
                fixedColor: Theme.of(context).primaryColor,
                onTap: _onItemTapped,
                unselectedItemColor: Color(0xff666666),
                type: BottomNavigationBarType.fixed),
          );
        });
  }
}
```

```dart
enum MenuType { home, sub, channels, my }

extension ResponseStatusExtension on MenuType {
  static const menuValue = {
    MenuType.my: 1,
    MenuType.home: 0,
  };

  int? get value => menuValue[this];
}
```

```dart
import 'dart:async';

import 'package:flutter/cupertino.dart';
import 'package:in_app_purchase/in_app_purchase.dart';
import 'package:wheel/wheel.dart' show AppLogHandler, GlobalConfig;
import 'package:wheel/wheel.dart';

import '../../widgets/app_page.dart';

final pageStorageBucket = PageStorageBucket();
final InAppPurchase inAppPurchase = InAppPurchase.instance;

class SleepGlobalConfig {
  static void loadApp(ConfigType configType) async {
    GlobalConfig.init(configType);
    void _handleError(Object obj, StackTrace stack) {
      AppLogHandler.logErrorStack("global error", obj, stack);
    }

    runZonedGuarded(() {
      FlutterError.onError = (FlutterErrorDetails errorDetails) {
        AppLogHandler.logFlutterErrorDetails(errorDetails);
      };
      runApp(AppPage());
    }, (Object error, StackTrace stackTrace) {
      _handleError(error, stackTrace);
    });
  }
}
```

```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:sleepease/generated/locales.g.dart';
import 'package:wheel/wheel.dart';

import '../page/nav/nav.dart';
import '../page/settings/setting/setting_controller.dart';
import 'global_controller.dart';

class AppPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final currentTheme = ThemeManager.fromThemeName("lightTheme");
    Get.put(SettingController());

    Widget buildHomePage() {
      return new Nav();
    }

    return GetBuilder<GlobalController>(
        init: GlobalController(),
        builder: (controller) {
          return GetMaterialApp(
            title: 'SleepEase',
            theme: currentTheme,
            navigatorKey: NavigationService.instance.navigationKey,
            checkerboardOffscreenLayers: controller.showDebug,
            checkerboardRasterCacheImages: controller.showDebug,
            showPerformanceOverlay: controller.showDebug,
            translationsKeys: AppTranslation.translations,
            locale: Locale('zh', 'CN'),
            fallbackLocale:Locale('en', 'US'),
            routes: {

            },
            home: buildHomePage(),
            onGenerateRoute: (RouteSettings settings) {},
          );
        });
  }
}
```

```dart
import 'package:get/get.dart';

class GlobalController extends GetxController {
  bool _showDebug = false;
  bool get showDebug => _showDebug;
  var appBarTitle = "首页".obs;

  void increment() {
    _showDebug = !_showDebug;
    update();
  }
}
```

```dart
//
// Generated file. Do not edit.
// This file is generated from template in file `flutter_tools/lib/src/flutter_plugins.dart`.
//

// @dart = 2.14

import 'dart:io'; // flutter_ignore: dart_io_import.
import 'package:path_provider_android/path_provider_android.dart';
import 'package:path_provider_foundation/path_provider_foundation.dart';
import 'package:device_info_plus_linux/device_info_plus_linux.dart';
import 'package:path_provider_linux/path_provider_linux.dart';
import 'package:shared_preferences_linux/shared_preferences_linux.dart';
import 'package:path_provider_foundation/path_provider_foundation.dart';
import 'package:shared_preferences_macos/shared_preferences_macos.dart';
import 'package:device_info_plus_windows/device_info_plus_windows.dart';
import 'package:path_provider_windows/path_provider_windows.dart';
import 'package:shared_preferences_windows/shared_preferences_windows.dart';

@pragma('vm:entry-point')
class _PluginRegistrant {

  @pragma('vm:entry-point')
  static void register() {
    if (Platform.isAndroid) {
      try {
        PathProviderAndroid.registerWith();
      } catch (err) {
        print(
          '`path_provider_android` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

    } else if (Platform.isIOS) {
      try {
        PathProviderFoundation.registerWith();
      } catch (err) {
        print(
          '`path_provider_foundation` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

    } else if (Platform.isLinux) {
      try {
        DeviceInfoLinux.registerWith();
      } catch (err) {
        print(
          '`device_info_plus_linux` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

      try {
        PathProviderLinux.registerWith();
      } catch (err) {
        print(
          '`path_provider_linux` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

      try {
        SharedPreferencesLinux.registerWith();
      } catch (err) {
        print(
          '`shared_preferences_linux` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

    } else if (Platform.isMacOS) {
      try {
        PathProviderFoundation.registerWith();
      } catch (err) {
        print(
          '`path_provider_foundation` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

      try {
        SharedPreferencesMacOS.registerWith();
      } catch (err) {
        print(
          '`shared_preferences_macos` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

    } else if (Platform.isWindows) {
      try {
        DeviceInfoWindows.registerWith();
      } catch (err) {
        print(
          '`device_info_plus_windows` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

      try {
        PathProviderWindows.registerWith();
      } catch (err) {
        print(
          '`path_provider_windows` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

      try {
        SharedPreferencesWindows.registerWith();
      } catch (err) {
        print(
          '`shared_preferences_windows` threw an error: $err. '
          'The app may not function as expected until you remove this plugin from pubspec.yaml'
        );
        rethrow;
      }

    }
  }
}
```

```dart
// Flutter web plugin registrant file.
//
// Generated file. Do not edit.
//

// @dart = 2.13
// ignore_for_file: type=lint

import 'package:audioplayers_web/audioplayers_web.dart';
import 'package:device_info_plus_web/device_info_plus_web.dart';
import 'package:flutter_secure_storage_web/flutter_secure_storage_web.dart';
import 'package:fluttertoast/fluttertoast_web.dart';
import 'package:shared_preferences_web/shared_preferences_web.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';

void registerPlugins([final Registrar? pluginRegistrar]) {
  final Registrar registrar = pluginRegistrar ?? webPluginRegistrar;
  AudioplayersPlugin.registerWith(registrar);
  DeviceInfoPlusPlugin.registerWith(registrar);
  FlutterSecureStorageWeb.registerWith(registrar);
  FluttertoastWebPlugin.registerWith(registrar);
  SharedPreferencesPlugin.registerWith(registrar);
  registrar.registerMessageHandler();
}
```

```dart
import 'app_localizations.dart';

/// The translations for French (`fr`).
class AppLocalizationsFr extends AppLocalizations {
  AppLocalizationsFr([String locale = 'fr']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for French, as used in Canada (`fr_CA`).
class AppLocalizationsFrCa extends AppLocalizationsFr {
  AppLocalizationsFrCa(): super('fr_CA');


}

/// The translations for French, as used in Switzerland (`fr_CH`).
class AppLocalizationsFrCh extends AppLocalizationsFr {
  AppLocalizationsFrCh(): super('fr_CH');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Bosnian (`bs`).
class AppLocalizationsBs extends AppLocalizations {
  AppLocalizationsBs([String locale = 'bs']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Nepali (`ne`).
class AppLocalizationsNe extends AppLocalizations {
  AppLocalizationsNe([String locale = 'ne']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Estonian (`et`).
class AppLocalizationsEt extends AppLocalizations {
  AppLocalizationsEt([String locale = 'et']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Zulu (`zu`).
class AppLocalizationsZu extends AppLocalizations {
  AppLocalizationsZu([String locale = 'zu']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Albanian (`sq`).
class AppLocalizationsSq extends AppLocalizations {
  AppLocalizationsSq([String locale = 'sq']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Kannada (`kn`).
class AppLocalizationsKn extends AppLocalizations {
  AppLocalizationsKn([String locale = 'kn']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_af.dart';
import 'app_localizations_am.dart';
import 'app_localizations_ar.dart';
import 'app_localizations_as.dart';
import 'app_localizations_az.dart';
import 'app_localizations_be.dart';
import 'app_localizations_bg.dart';
import 'app_localizations_bn.dart';
import 'app_localizations_bs.dart';
import 'app_localizations_ca.dart';
import 'app_localizations_cs.dart';
import 'app_localizations_da.dart';
import 'app_localizations_de.dart';
import 'app_localizations_el.dart';
import 'app_localizations_en.dart';
import 'app_localizations_es.dart';
import 'app_localizations_et.dart';
import 'app_localizations_eu.dart';
import 'app_localizations_fa.dart';
import 'app_localizations_fi.dart';
import 'app_localizations_fil.dart';
import 'app_localizations_fr.dart';
import 'app_localizations_gl.dart';
import 'app_localizations_gsw.dart';
import 'app_localizations_gu.dart';
import 'app_localizations_he.dart';
import 'app_localizations_hi.dart';
import 'app_localizations_hr.dart';
import 'app_localizations_hu.dart';
import 'app_localizations_hy.dart';
import 'app_localizations_id.dart';
import 'app_localizations_is.dart';
import 'app_localizations_it.dart';
import 'app_localizations_ja.dart';
import 'app_localizations_ka.dart';
import 'app_localizations_kk.dart';
import 'app_localizations_km.dart';
import 'app_localizations_kn.dart';
import 'app_localizations_ko.dart';
import 'app_localizations_ky.dart';
import 'app_localizations_lo.dart';
import 'app_localizations_lt.dart';
import 'app_localizations_lv.dart';
import 'app_localizations_mk.dart';
import 'app_localizations_ml.dart';
import 'app_localizations_mn.dart';
import 'app_localizations_mr.dart';
import 'app_localizations_ms.dart';
import 'app_localizations_my.dart';
import 'app_localizations_nb.dart';
import 'app_localizations_ne.dart';
import 'app_localizations_nl.dart';
import 'app_localizations_or.dart';
import 'app_localizations_pa.dart';
import 'app_localizations_pl.dart';
import 'app_localizations_pt.dart';
import 'app_localizations_ro.dart';
import 'app_localizations_ru.dart';
import 'app_localizations_si.dart';
import 'app_localizations_sk.dart';
import 'app_localizations_sl.dart';
import 'app_localizations_sq.dart';
import 'app_localizations_sr.dart';
import 'app_localizations_sv.dart';
import 'app_localizations_sw.dart';
import 'app_localizations_ta.dart';
import 'app_localizations_te.dart';
import 'app_localizations_th.dart';
import 'app_localizations_tl.dart';
import 'app_localizations_tr.dart';
import 'app_localizations_uk.dart';
import 'app_localizations_ur.dart';
import 'app_localizations_uz.dart';
import 'app_localizations_vi.dart';
import 'app_localizations_zh.dart';
import 'app_localizations_zu.dart';

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'gen_l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale) : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations)!;
  }

  static const LocalizationsDelegate<AppLocalizations> delegate = _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates = <LocalizationsDelegate<dynamic>>[
    delegate,
    GlobalMaterialLocalizations.delegate,
    GlobalCupertinoLocalizations.delegate,
    GlobalWidgetsLocalizations.delegate,
  ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('af'),
    Locale('am'),
    Locale('ar'),
    Locale('ar', 'EG'),
    Locale('ar', 'JO'),
    Locale('ar', 'MA'),
    Locale('ar', 'SA'),
    Locale('as'),
    Locale('az'),
    Locale('be'),
    Locale('bg'),
    Locale('bn'),
    Locale('bs'),
    Locale('ca'),
    Locale('cs'),
    Locale('da'),
    Locale('de'),
    Locale('de', 'AT'),
    Locale('de', 'CH'),
    Locale('el'),
    Locale('en', 'AU'),
    Locale('en', 'CA'),
    Locale('en', 'GB'),
    Locale('en', 'IE'),
    Locale('en', 'IN'),
    Locale('en', 'NZ'),
    Locale('en', 'SG'),
    Locale('en', 'ZA'),
    Locale('es'),
    Locale('es', '419'),
    Locale('es', 'AR'),
    Locale('es', 'BO'),
    Locale('es', 'CL'),
    Locale('es', 'CO'),
    Locale('es', 'CR'),
    Locale('es', 'DO'),
    Locale('es', 'EC'),
    Locale('es', 'GT'),
    Locale('es', 'HN'),
    Locale('es', 'MX'),
    Locale('es', 'NI'),
    Locale('es', 'PA'),
    Locale('es', 'PE'),
    Locale('es', 'PR'),
    Locale('es', 'PY'),
    Locale('es', 'SV'),
    Locale('es', 'US'),
    Locale('es', 'UY'),
    Locale('es', 'VE'),
    Locale('et'),
    Locale('eu'),
    Locale('fa'),
    Locale('fi'),
    Locale('fil'),
    Locale('fr'),
    Locale('fr', 'CA'),
    Locale('fr', 'CH'),
    Locale('gl'),
    Locale('gsw'),
    Locale('gu'),
    Locale('he'),
    Locale('hi'),
    Locale('hr'),
    Locale('hu'),
    Locale('hy'),
    Locale('id'),
    Locale('is'),
    Locale('it'),
    Locale('ja'),
    Locale('ka'),
    Locale('kk'),
    Locale('km'),
    Locale('kn'),
    Locale('ko'),
    Locale('ky'),
    Locale('lo'),
    Locale('lt'),
    Locale('lv'),
    Locale('mk'),
    Locale('ml'),
    Locale('mn'),
    Locale('mr'),
    Locale('ms'),
    Locale('my'),
    Locale('nb'),
    Locale('ne'),
    Locale('nl'),
    Locale('or'),
    Locale('pa'),
    Locale('pl'),
    Locale('pt'),
    Locale('pt', 'BR'),
    Locale('pt', 'PT'),
    Locale('ro'),
    Locale('ru'),
    Locale('si'),
    Locale('sk'),
    Locale('sl'),
    Locale('sq'),
    Locale('sr'),
    Locale.fromSubtags(languageCode: 'sr', scriptCode: 'Latn'),
    Locale('sv'),
    Locale('sw'),
    Locale('ta'),
    Locale('te'),
    Locale('th'),
    Locale('tl'),
    Locale('tr'),
    Locale('uk'),
    Locale('ur'),
    Locale('uz'),
    Locale('vi'),
    Locale('zh'),
    Locale('zh', 'CN'),
    Locale('zh', 'HK'),
    Locale('zh', 'TW'),
    Locale('zu')
  ];

  /// The conventional newborn programmer greeting
  ///
  /// In en, this message translates to:
  /// **'Hello World!'**
  String get helloWorld;

  /// The home navigate text
  ///
  /// In en, this message translates to:
  /// **'Home'**
  String get cruiseNavigatorHome;

  /// The subscribe navigate text
  ///
  /// In en, this message translates to:
  /// **'Subscribe'**
  String get cruiseNavigatorSubscribe;

  /// All Channel
  ///
  /// In en, this message translates to:
  /// **'Channel'**
  String get cruiseNavigatorChannel;

  /// My Space
  ///
  /// In en, this message translates to:
  /// **'My'**
  String get cruiseNavigatorMine;
}

class _AppLocalizationsDelegate extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) => <String>['en', 'af', 'am', 'ar', 'as', 'az', 'be', 'bg', 'bn', 'bs', 'ca', 'cs', 'da', 'de', 'el', 'es', 'et', 'eu', 'fa', 'fi', 'fil', 'fr', 'gl', 'gsw', 'gu', 'he', 'hi', 'hr', 'hu', 'hy', 'id', 'is', 'it', 'ja', 'ka', 'kk', 'km', 'kn', 'ko', 'ky', 'lo', 'lt', 'lv', 'mk', 'ml', 'mn', 'mr', 'ms', 'my', 'nb', 'ne', 'nl', 'or', 'pa', 'pl', 'pt', 'ro', 'ru', 'si', 'sk', 'sl', 'sq', 'sr', 'sv', 'sw', 'ta', 'te', 'th', 'tl', 'tr', 'uk', 'ur', 'uz', 'vi', 'zh', 'zu'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {

  // Lookup logic when language+script codes are specified.
  switch (locale.languageCode) {
    case 'sr': {
  switch (locale.scriptCode) {
    case 'Latn': return AppLocalizationsSrLatn();
   }
  break;
   }
  }

  // Lookup logic when language+country codes are specified.
  switch (locale.languageCode) {
    case 'en': {
  switch (locale.countryCode) {
    case 'AU': return AppLocalizationsEnAu();
case 'CA': return AppLocalizationsEnCa();
case 'GB': return AppLocalizationsEnGb();
case 'IE': return AppLocalizationsEnIe();
case 'IN': return AppLocalizationsEnIn();
case 'NZ': return AppLocalizationsEnNz();
case 'SG': return AppLocalizationsEnSg();
case 'ZA': return AppLocalizationsEnZa();
   }
  break;
   }
    case 'ar': {
  switch (locale.countryCode) {
    case 'EG': return AppLocalizationsArEg();
case 'JO': return AppLocalizationsArJo();
case 'MA': return AppLocalizationsArMa();
case 'SA': return AppLocalizationsArSa();
   }
  break;
   }
    case 'de': {
  switch (locale.countryCode) {
    case 'AT': return AppLocalizationsDeAt();
case 'CH': return AppLocalizationsDeCh();
   }
  break;
   }
    case 'es': {
  switch (locale.countryCode) {
    case '419': return AppLocalizationsEs419();
case 'AR': return AppLocalizationsEsAr();
case 'BO': return AppLocalizationsEsBo();
case 'CL': return AppLocalizationsEsCl();
case 'CO': return AppLocalizationsEsCo();
case 'CR': return AppLocalizationsEsCr();
case 'DO': return AppLocalizationsEsDo();
case 'EC': return AppLocalizationsEsEc();
case 'GT': return AppLocalizationsEsGt();
case 'HN': return AppLocalizationsEsHn();
case 'MX': return AppLocalizationsEsMx();
case 'NI': return AppLocalizationsEsNi();
case 'PA': return AppLocalizationsEsPa();
case 'PE': return AppLocalizationsEsPe();
case 'PR': return AppLocalizationsEsPr();
case 'PY': return AppLocalizationsEsPy();
case 'SV': return AppLocalizationsEsSv();
case 'US': return AppLocalizationsEsUs();
case 'UY': return AppLocalizationsEsUy();
case 'VE': return AppLocalizationsEsVe();
   }
  break;
   }
    case 'fr': {
  switch (locale.countryCode) {
    case 'CA': return AppLocalizationsFrCa();
case 'CH': return AppLocalizationsFrCh();
   }
  break;
   }
    case 'pt': {
  switch (locale.countryCode) {
    case 'BR': return AppLocalizationsPtBr();
case 'PT': return AppLocalizationsPtPt();
   }
  break;
   }
    case 'zh': {
  switch (locale.countryCode) {
    case 'CN': return AppLocalizationsZhCn();
case 'HK': return AppLocalizationsZhHk();
case 'TW': return AppLocalizationsZhTw();
   }
  break;
   }
  }

  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en': return AppLocalizationsEn();
    case 'af': return AppLocalizationsAf();
    case 'am': return AppLocalizationsAm();
    case 'ar': return AppLocalizationsAr();
    case 'as': return AppLocalizationsAs();
    case 'az': return AppLocalizationsAz();
    case 'be': return AppLocalizationsBe();
    case 'bg': return AppLocalizationsBg();
    case 'bn': return AppLocalizationsBn();
    case 'bs': return AppLocalizationsBs();
    case 'ca': return AppLocalizationsCa();
    case 'cs': return AppLocalizationsCs();
    case 'da': return AppLocalizationsDa();
    case 'de': return AppLocalizationsDe();
    case 'el': return AppLocalizationsEl();
    case 'es': return AppLocalizationsEs();
    case 'et': return AppLocalizationsEt();
    case 'eu': return AppLocalizationsEu();
    case 'fa': return AppLocalizationsFa();
    case 'fi': return AppLocalizationsFi();
    case 'fil': return AppLocalizationsFil();
    case 'fr': return AppLocalizationsFr();
    case 'gl': return AppLocalizationsGl();
    case 'gsw': return AppLocalizationsGsw();
    case 'gu': return AppLocalizationsGu();
    case 'he': return AppLocalizationsHe();
    case 'hi': return AppLocalizationsHi();
    case 'hr': return AppLocalizationsHr();
    case 'hu': return AppLocalizationsHu();
    case 'hy': return AppLocalizationsHy();
    case 'id': return AppLocalizationsId();
    case 'is': return AppLocalizationsIs();
    case 'it': return AppLocalizationsIt();
    case 'ja': return AppLocalizationsJa();
    case 'ka': return AppLocalizationsKa();
    case 'kk': return AppLocalizationsKk();
    case 'km': return AppLocalizationsKm();
    case 'kn': return AppLocalizationsKn();
    case 'ko': return AppLocalizationsKo();
    case 'ky': return AppLocalizationsKy();
    case 'lo': return AppLocalizationsLo();
    case 'lt': return AppLocalizationsLt();
    case 'lv': return AppLocalizationsLv();
    case 'mk': return AppLocalizationsMk();
    case 'ml': return AppLocalizationsMl();
    case 'mn': return AppLocalizationsMn();
    case 'mr': return AppLocalizationsMr();
    case 'ms': return AppLocalizationsMs();
    case 'my': return AppLocalizationsMy();
    case 'nb': return AppLocalizationsNb();
    case 'ne': return AppLocalizationsNe();
    case 'nl': return AppLocalizationsNl();
    case 'or': return AppLocalizationsOr();
    case 'pa': return AppLocalizationsPa();
    case 'pl': return AppLocalizationsPl();
    case 'pt': return AppLocalizationsPt();
    case 'ro': return AppLocalizationsRo();
    case 'ru': return AppLocalizationsRu();
    case 'si': return AppLocalizationsSi();
    case 'sk': return AppLocalizationsSk();
    case 'sl': return AppLocalizationsSl();
    case 'sq': return AppLocalizationsSq();
    case 'sr': return AppLocalizationsSr();
    case 'sv': return AppLocalizationsSv();
    case 'sw': return AppLocalizationsSw();
    case 'ta': return AppLocalizationsTa();
    case 'te': return AppLocalizationsTe();
    case 'th': return AppLocalizationsTh();
    case 'tl': return AppLocalizationsTl();
    case 'tr': return AppLocalizationsTr();
    case 'uk': return AppLocalizationsUk();
    case 'ur': return AppLocalizationsUr();
    case 'uz': return AppLocalizationsUz();
    case 'vi': return AppLocalizationsVi();
    case 'zh': return AppLocalizationsZh();
    case 'zu': return AppLocalizationsZu();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.'
  );
}
```

```dart
import 'app_localizations.dart';

/// The translations for Korean (`ko`).
class AppLocalizationsKo extends AppLocalizations {
  AppLocalizationsKo([String locale = 'ko']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Hebrew (`he`).
class AppLocalizationsHe extends AppLocalizations {
  AppLocalizationsHe([String locale = 'he']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Basque (`eu`).
class AppLocalizationsEu extends AppLocalizations {
  AppLocalizationsEu([String locale = 'eu']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Galician (`gl`).
class AppLocalizationsGl extends AppLocalizations {
  AppLocalizationsGl([String locale = 'gl']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Gujarati (`gu`).
class AppLocalizationsGu extends AppLocalizations {
  AppLocalizationsGu([String locale = 'gu']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Modern Greek (`el`).
class AppLocalizationsEl extends AppLocalizations {
  AppLocalizationsEl([String locale = 'el']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Amharic (`am`).
class AppLocalizationsAm extends AppLocalizations {
  AppLocalizationsAm([String locale = 'am']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Hindi (`hi`).
class AppLocalizationsHi extends AppLocalizations {
  AppLocalizationsHi([String locale = 'hi']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Sinhala Sinhalese (`si`).
class AppLocalizationsSi extends AppLocalizations {
  AppLocalizationsSi([String locale = 'si']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Mongolian (`mn`).
class AppLocalizationsMn extends AppLocalizations {
  AppLocalizationsMn([String locale = 'mn']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Catalan Valencian (`ca`).
class AppLocalizationsCa extends AppLocalizations {
  AppLocalizationsCa([String locale = 'ca']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Romanian Moldavian Moldovan (`ro`).
class AppLocalizationsRo extends AppLocalizations {
  AppLocalizationsRo([String locale = 'ro']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Bulgarian (`bg`).
class AppLocalizationsBg extends AppLocalizations {
  AppLocalizationsBg([String locale = 'bg']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Uzbek (`uz`).
class AppLocalizationsUz extends AppLocalizations {
  AppLocalizationsUz([String locale = 'uz']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Indonesian (`id`).
class AppLocalizationsId extends AppLocalizations {
  AppLocalizationsId([String locale = 'id']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Assamese (`as`).
class AppLocalizationsAs extends AppLocalizations {
  AppLocalizationsAs([String locale = 'as']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Swahili (`sw`).
class AppLocalizationsSw extends AppLocalizations {
  AppLocalizationsSw([String locale = 'sw']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Afrikaans (`af`).
class AppLocalizationsAf extends AppLocalizations {
  AppLocalizationsAf([String locale = 'af']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Danish (`da`).
class AppLocalizationsDa extends AppLocalizations {
  AppLocalizationsDa([String locale = 'da']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Latvian (`lv`).
class AppLocalizationsLv extends AppLocalizations {
  AppLocalizationsLv([String locale = 'lv']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Lao (`lo`).
class AppLocalizationsLo extends AppLocalizations {
  AppLocalizationsLo([String locale = 'lo']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Persian (`fa`).
class AppLocalizationsFa extends AppLocalizations {
  AppLocalizationsFa([String locale = 'fa']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Vietnamese (`vi`).
class AppLocalizationsVi extends AppLocalizations {
  AppLocalizationsVi([String locale = 'vi']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Czech (`cs`).
class AppLocalizationsCs extends AppLocalizations {
  AppLocalizationsCs([String locale = 'cs']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Telugu (`te`).
class AppLocalizationsTe extends AppLocalizations {
  AppLocalizationsTe([String locale = 'te']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Norwegian Bokmål (`nb`).
class AppLocalizationsNb extends AppLocalizations {
  AppLocalizationsNb([String locale = 'nb']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Thai (`th`).
class AppLocalizationsTh extends AppLocalizations {
  AppLocalizationsTh([String locale = 'th']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Swedish (`sv`).
class AppLocalizationsSv extends AppLocalizations {
  AppLocalizationsSv([String locale = 'sv']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Arabic (`ar`).
class AppLocalizationsAr extends AppLocalizations {
  AppLocalizationsAr([String locale = 'ar']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for Arabic, as used in Egypt (`ar_EG`).
class AppLocalizationsArEg extends AppLocalizationsAr {
  AppLocalizationsArEg(): super('ar_EG');


}

/// The translations for Arabic, as used in Jordan (`ar_JO`).
class AppLocalizationsArJo extends AppLocalizationsAr {
  AppLocalizationsArJo(): super('ar_JO');


}

/// The translations for Arabic, as used in Morocco (`ar_MA`).
class AppLocalizationsArMa extends AppLocalizationsAr {
  AppLocalizationsArMa(): super('ar_MA');


}

/// The translations for Arabic, as used in Saudi Arabia (`ar_SA`).
class AppLocalizationsArSa extends AppLocalizationsAr {
  AppLocalizationsArSa(): super('ar_SA');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Spanish Castilian (`es`).
class AppLocalizationsEs extends AppLocalizations {
  AppLocalizationsEs([String locale = 'es']) : super(locale);

  @override
  String get helloWorld => 'Hola Mundo!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for Spanish Castilian, as used in Latin America and the Caribbean (`es_419`).
class AppLocalizationsEs419 extends AppLocalizationsEs {
  AppLocalizationsEs419(): super('es_419');


}

/// The translations for Spanish Castilian, as used in Argentina (`es_AR`).
class AppLocalizationsEsAr extends AppLocalizationsEs {
  AppLocalizationsEsAr(): super('es_AR');


}

/// The translations for Spanish Castilian, as used in Bolivia (`es_BO`).
class AppLocalizationsEsBo extends AppLocalizationsEs {
  AppLocalizationsEsBo(): super('es_BO');


}

/// The translations for Spanish Castilian, as used in Chile (`es_CL`).
class AppLocalizationsEsCl extends AppLocalizationsEs {
  AppLocalizationsEsCl(): super('es_CL');


}

/// The translations for Spanish Castilian, as used in Colombia (`es_CO`).
class AppLocalizationsEsCo extends AppLocalizationsEs {
  AppLocalizationsEsCo(): super('es_CO');


}

/// The translations for Spanish Castilian, as used in Costa Rica (`es_CR`).
class AppLocalizationsEsCr extends AppLocalizationsEs {
  AppLocalizationsEsCr(): super('es_CR');


}

/// The translations for Spanish Castilian, as used in the Dominican Republic (`es_DO`).
class AppLocalizationsEsDo extends AppLocalizationsEs {
  AppLocalizationsEsDo(): super('es_DO');


}

/// The translations for Spanish Castilian, as used in Ecuador (`es_EC`).
class AppLocalizationsEsEc extends AppLocalizationsEs {
  AppLocalizationsEsEc(): super('es_EC');


}

/// The translations for Spanish Castilian, as used in Guatemala (`es_GT`).
class AppLocalizationsEsGt extends AppLocalizationsEs {
  AppLocalizationsEsGt(): super('es_GT');


}

/// The translations for Spanish Castilian, as used in Honduras (`es_HN`).
class AppLocalizationsEsHn extends AppLocalizationsEs {
  AppLocalizationsEsHn(): super('es_HN');


}

/// The translations for Spanish Castilian, as used in Mexico (`es_MX`).
class AppLocalizationsEsMx extends AppLocalizationsEs {
  AppLocalizationsEsMx(): super('es_MX');


}

/// The translations for Spanish Castilian, as used in Nicaragua (`es_NI`).
class AppLocalizationsEsNi extends AppLocalizationsEs {
  AppLocalizationsEsNi(): super('es_NI');


}

/// The translations for Spanish Castilian, as used in Panama (`es_PA`).
class AppLocalizationsEsPa extends AppLocalizationsEs {
  AppLocalizationsEsPa(): super('es_PA');


}

/// The translations for Spanish Castilian, as used in Peru (`es_PE`).
class AppLocalizationsEsPe extends AppLocalizationsEs {
  AppLocalizationsEsPe(): super('es_PE');


}

/// The translations for Spanish Castilian, as used in Puerto Rico (`es_PR`).
class AppLocalizationsEsPr extends AppLocalizationsEs {
  AppLocalizationsEsPr(): super('es_PR');


}

/// The translations for Spanish Castilian, as used in Paraguay (`es_PY`).
class AppLocalizationsEsPy extends AppLocalizationsEs {
  AppLocalizationsEsPy(): super('es_PY');


}

/// The translations for Spanish Castilian, as used in El Salvador (`es_SV`).
class AppLocalizationsEsSv extends AppLocalizationsEs {
  AppLocalizationsEsSv(): super('es_SV');


}

/// The translations for Spanish Castilian, as used in the United States (`es_US`).
class AppLocalizationsEsUs extends AppLocalizationsEs {
  AppLocalizationsEsUs(): super('es_US');


}

/// The translations for Spanish Castilian, as used in Uruguay (`es_UY`).
class AppLocalizationsEsUy extends AppLocalizationsEs {
  AppLocalizationsEsUy(): super('es_UY');


}

/// The translations for Spanish Castilian, as used in Venezuela (`es_VE`).
class AppLocalizationsEsVe extends AppLocalizationsEs {
  AppLocalizationsEsVe(): super('es_VE');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Lithuanian (`lt`).
class AppLocalizationsLt extends AppLocalizations {
  AppLocalizationsLt([String locale = 'lt']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Icelandic (`is`).
class AppLocalizationsIs extends AppLocalizations {
  AppLocalizationsIs([String locale = 'is']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Hungarian (`hu`).
class AppLocalizationsHu extends AppLocalizations {
  AppLocalizationsHu([String locale = 'hu']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Marathi (`mr`).
class AppLocalizationsMr extends AppLocalizations {
  AppLocalizationsMr([String locale = 'mr']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Bengali Bangla (`bn`).
class AppLocalizationsBn extends AppLocalizations {
  AppLocalizationsBn([String locale = 'bn']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Chinese (`zh`).
class AppLocalizationsZh extends AppLocalizations {
  AppLocalizationsZh([String locale = 'zh']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => '首页';

  @override
  String get cruiseNavigatorSubscribe => '关注';

  @override
  String get cruiseNavigatorChannel => '频道';

  @override
  String get cruiseNavigatorMine => '我的';
}

/// The translations for Chinese, as used in China (`zh_CN`).
class AppLocalizationsZhCn extends AppLocalizationsZh {
  AppLocalizationsZhCn(): super('zh_CN');

  @override
  String get cruiseNavigatorHome => '首页';

  @override
  String get cruiseNavigatorSubscribe => '关注';
}

/// The translations for Chinese, as used in Hong Kong (`zh_HK`).
class AppLocalizationsZhHk extends AppLocalizationsZh {
  AppLocalizationsZhHk(): super('zh_HK');

  @override
  String get cruiseNavigatorHome => '首页';

  @override
  String get cruiseNavigatorSubscribe => '關注';
}

/// The translations for Chinese, as used in Taiwan (`zh_TW`).
class AppLocalizationsZhTw extends AppLocalizationsZh {
  AppLocalizationsZhTw(): super('zh_TW');

  @override
  String get cruiseNavigatorHome => '首页';

  @override
  String get cruiseNavigatorSubscribe => '關注';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Armenian (`hy`).
class AppLocalizationsHy extends AppLocalizations {
  AppLocalizationsHy([String locale = 'hy']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Turkish (`tr`).
class AppLocalizationsTr extends AppLocalizations {
  AppLocalizationsTr([String locale = 'tr']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Slovenian (`sl`).
class AppLocalizationsSl extends AppLocalizations {
  AppLocalizationsSl([String locale = 'sl']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Oriya (`or`).
class AppLocalizationsOr extends AppLocalizations {
  AppLocalizationsOr([String locale = 'or']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Macedonian (`mk`).
class AppLocalizationsMk extends AppLocalizations {
  AppLocalizationsMk([String locale = 'mk']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Japanese (`ja`).
class AppLocalizationsJa extends AppLocalizations {
  AppLocalizationsJa([String locale = 'ja']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Malay (`ms`).
class AppLocalizationsMs extends AppLocalizations {
  AppLocalizationsMs([String locale = 'ms']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Kazakh (`kk`).
class AppLocalizationsKk extends AppLocalizations {
  AppLocalizationsKk([String locale = 'kk']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Dutch Flemish (`nl`).
class AppLocalizationsNl extends AppLocalizations {
  AppLocalizationsNl([String locale = 'nl']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Tamil (`ta`).
class AppLocalizationsTa extends AppLocalizations {
  AppLocalizationsTa([String locale = 'ta']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Swiss German Alemannic Alsatian (`gsw`).
class AppLocalizationsGsw extends AppLocalizations {
  AppLocalizationsGsw([String locale = 'gsw']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Urdu (`ur`).
class AppLocalizationsUr extends AppLocalizations {
  AppLocalizationsUr([String locale = 'ur']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Russian (`ru`).
class AppLocalizationsRu extends AppLocalizations {
  AppLocalizationsRu([String locale = 'ru']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Polish (`pl`).
class AppLocalizationsPl extends AppLocalizations {
  AppLocalizationsPl([String locale = 'pl']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Ukrainian (`uk`).
class AppLocalizationsUk extends AppLocalizations {
  AppLocalizationsUk([String locale = 'uk']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Filipino Pilipino (`fil`).
class AppLocalizationsFil extends AppLocalizations {
  AppLocalizationsFil([String locale = 'fil']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for German (`de`).
class AppLocalizationsDe extends AppLocalizations {
  AppLocalizationsDe([String locale = 'de']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for German, as used in Austria (`de_AT`).
class AppLocalizationsDeAt extends AppLocalizationsDe {
  AppLocalizationsDeAt(): super('de_AT');


}

/// The translations for German, as used in Switzerland (`de_CH`).
class AppLocalizationsDeCh extends AppLocalizationsDe {
  AppLocalizationsDeCh(): super('de_CH');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Finnish (`fi`).
class AppLocalizationsFi extends AppLocalizations {
  AppLocalizationsFi([String locale = 'fi']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Kirghiz Kyrgyz (`ky`).
class AppLocalizationsKy extends AppLocalizations {
  AppLocalizationsKy([String locale = 'ky']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Serbian (`sr`).
class AppLocalizationsSr extends AppLocalizations {
  AppLocalizationsSr([String locale = 'sr']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for Serbian, using the Latin script (`sr_Latn`).
class AppLocalizationsSrLatn extends AppLocalizationsSr {
  AppLocalizationsSrLatn(): super('sr_Latn');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Tagalog (`tl`).
class AppLocalizationsTl extends AppLocalizations {
  AppLocalizationsTl([String locale = 'tl']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Croatian (`hr`).
class AppLocalizationsHr extends AppLocalizations {
  AppLocalizationsHr([String locale = 'hr']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Khmer Central Khmer (`km`).
class AppLocalizationsKm extends AppLocalizations {
  AppLocalizationsKm([String locale = 'km']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Italian (`it`).
class AppLocalizationsIt extends AppLocalizations {
  AppLocalizationsIt([String locale = 'it']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Belarusian (`be`).
class AppLocalizationsBe extends AppLocalizations {
  AppLocalizationsBe([String locale = 'be']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Portuguese (`pt`).
class AppLocalizationsPt extends AppLocalizations {
  AppLocalizationsPt([String locale = 'pt']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for Portuguese, as used in Brazil (`pt_BR`).
class AppLocalizationsPtBr extends AppLocalizationsPt {
  AppLocalizationsPtBr(): super('pt_BR');


}

/// The translations for Portuguese, as used in Portugal (`pt_PT`).
class AppLocalizationsPtPt extends AppLocalizationsPt {
  AppLocalizationsPtPt(): super('pt_PT');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Azerbaijani (`az`).
class AppLocalizationsAz extends AppLocalizations {
  AppLocalizationsAz([String locale = 'az']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Malayalam (`ml`).
class AppLocalizationsMl extends AppLocalizations {
  AppLocalizationsMl([String locale = 'ml']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Slovak (`sk`).
class AppLocalizationsSk extends AppLocalizations {
  AppLocalizationsSk([String locale = 'sk']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Burmese (`my`).
class AppLocalizationsMy extends AppLocalizations {
  AppLocalizationsMy([String locale = 'my']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for Georgian (`ka`).
class AppLocalizationsKa extends AppLocalizations {
  AppLocalizationsKa([String locale = 'ka']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'app_localizations.dart';

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}

/// The translations for English, as used in Australia (`en_AU`).
class AppLocalizationsEnAu extends AppLocalizationsEn {
  AppLocalizationsEnAu(): super('en_AU');


}

/// The translations for English, as used in Canada (`en_CA`).
class AppLocalizationsEnCa extends AppLocalizationsEn {
  AppLocalizationsEnCa(): super('en_CA');


}

/// The translations for English, as used in the United Kingdom (`en_GB`).
class AppLocalizationsEnGb extends AppLocalizationsEn {
  AppLocalizationsEnGb(): super('en_GB');


}

/// The translations for English, as used in Ireland (`en_IE`).
class AppLocalizationsEnIe extends AppLocalizationsEn {
  AppLocalizationsEnIe(): super('en_IE');


}

/// The translations for English, as used in India (`en_IN`).
class AppLocalizationsEnIn extends AppLocalizationsEn {
  AppLocalizationsEnIn(): super('en_IN');


}

/// The translations for English, as used in New Zealand (`en_NZ`).
class AppLocalizationsEnNz extends AppLocalizationsEn {
  AppLocalizationsEnNz(): super('en_NZ');


}

/// The translations for English, as used in Singapore (`en_SG`).
class AppLocalizationsEnSg extends AppLocalizationsEn {
  AppLocalizationsEnSg(): super('en_SG');


}

/// The translations for English, as used in South Africa (`en_ZA`).
class AppLocalizationsEnZa extends AppLocalizationsEn {
  AppLocalizationsEnZa(): super('en_ZA');


}
```

```dart
import 'app_localizations.dart';

/// The translations for Panjabi Punjabi (`pa`).
class AppLocalizationsPa extends AppLocalizations {
  AppLocalizationsPa([String locale = 'pa']) : super(locale);

  @override
  String get helloWorld => 'Hello World!';

  @override
  String get cruiseNavigatorHome => 'Home';

  @override
  String get cruiseNavigatorSubscribe => 'Subscribe';

  @override
  String get cruiseNavigatorChannel => 'Channel';

  @override
  String get cruiseNavigatorMine => 'My';
}
```

```dart
import 'package:device_info_plus_platform_interface/device_info_plus_platform_interface.dart';
import 'package:device_info_plus_windows/src/device_info_plus_windows.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('registered instance', () {
    DeviceInfoWindows.registerWith();
    expect(DeviceInfoPlatform.instance, isA<DeviceInfoWindows>());
  });
}
```

```dart
export 'src/device_info_plus_windows.dart';
```

```dart
/// The Windows implementation of `device_info_plus`.
library device_info_plus_windows;

import 'dart:ffi';

import 'package:device_info_plus_platform_interface/device_info_plus_platform_interface.dart';
import 'package:ffi/ffi.dart';
import 'package:win32/win32.dart';

/// The Windows implementation of [DeviceInfoPlatform].
class DeviceInfoWindows extends DeviceInfoPlatform {
  /// Register this dart class as the platform implementation for windows
  static void registerWith() {
    DeviceInfoPlatform.instance = DeviceInfoWindows();
  }

  /// Returns a [WindowsDeviceInfo] with information about the device.
  @override
  Future<WindowsDeviceInfo> windowsInfo() {
    final system_info = _getInfoStructPointer();

    GetSystemInfo(system_info);

    final data = WindowsDeviceInfo(
      numberOfCores: system_info.ref.dwNumberOfProcessors,
      computerName: _getComputerName(),
      systemMemoryInMegabytes: _getSystemMemoryInMegabytes(),
    );
    calloc.free(system_info);
    return Future.value(data);
  }
}

int _getSystemMemoryInMegabytes() {
  final memory = calloc<Uint64>();

  try {
    final result = GetPhysicallyInstalledSystemMemory(memory);
    if (result != 0) {
      return memory.value ~/ 1024;
    } else {
      final error = GetLastError();
      throw WindowsException(HRESULT_FROM_WIN32(error));
    }
  } finally {
    calloc.free(memory);
  }
}

String _getComputerName() {
  final nameLength = calloc<Uint32>();
  String name;

  GetComputerNameEx(
      COMPUTER_NAME_FORMAT.ComputerNameDnsFullyQualified, nullptr, nameLength);

  final namePtr = calloc<Uint16>(nameLength.value).cast<Utf16>();

  try {
    final result = GetComputerNameEx(
        COMPUTER_NAME_FORMAT.ComputerNameDnsFullyQualified,
        namePtr,
        nameLength);

    if (result != 0) {
      name = namePtr.toDartString();
    } else {
      throw WindowsException(HRESULT_FROM_WIN32(GetLastError()));
    }
  } finally {
    calloc.free(namePtr);
    calloc.free(nameLength);
  }
  return name;
}

Pointer<SYSTEM_INFO> _getInfoStructPointer() {
  final pointer = calloc<SYSTEM_INFO>();
  pointer.ref
    ..wProcessorArchitecture = 0
    ..wReserved = 0
    ..dwPageSize = 0
    ..lpMaximumApplicationAddress = nullptr
    ..lpMaximumApplicationAddress = nullptr
    ..dwActiveProcessorMask = 0
    ..dwNumberOfProcessors = 0
    ..dwProcessorType = 0
    ..dwAllocationGranularity = 0
    ..wProcessorLevel = 0
    ..wProcessorRevision = 0;
  return pointer;
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:convert';

import 'package:file/memory.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:path_provider_windows/path_provider_windows.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_platform_interface.dart';
import 'package:shared_preferences_windows/shared_preferences_windows.dart';

void main() {
  late MemoryFileSystem fs;
  late PathProviderWindows pathProvider;

  SharedPreferencesWindows.registerWith();

  const Map<String, Object> flutterTestValues = <String, Object>{
    'flutter.String': 'hello world',
    'flutter.Bool': true,
    'flutter.Int': 42,
    'flutter.Double': 3.14159,
    'flutter.StringList': <String>['foo', 'bar'],
  };

  const Map<String, Object> prefixTestValues = <String, Object>{
    'prefix.String': 'hello world',
    'prefix.Bool': true,
    'prefix.Int': 42,
    'prefix.Double': 3.14159,
    'prefix.StringList': <String>['foo', 'bar'],
  };

  const Map<String, Object> nonPrefixTestValues = <String, Object>{
    'String': 'hello world',
    'Bool': true,
    'Int': 42,
    'Double': 3.14159,
    'StringList': <String>['foo', 'bar'],
  };

  final Map<String, Object> allTestValues = <String, Object>{};

  allTestValues.addAll(flutterTestValues);
  allTestValues.addAll(prefixTestValues);
  allTestValues.addAll(nonPrefixTestValues);

  setUp(() {
    fs = MemoryFileSystem.test();
    pathProvider = FakePathProviderWindows();
  });

  Future<String> getFilePath() async {
    final String? directory = await pathProvider.getApplicationSupportPath();
    return path.join(directory!, 'shared_preferences.json');
  }

  Future<void> writeTestFile(String value) async {
    fs.file(await getFilePath())
      ..createSync(recursive: true)
      ..writeAsStringSync(value);
  }

  Future<String> readTestFile() async {
    return fs.file(await getFilePath()).readAsStringSync();
  }

  SharedPreferencesWindows getPreferences() {
    final SharedPreferencesWindows prefs = SharedPreferencesWindows();
    prefs.fs = fs;
    prefs.pathProvider = pathProvider;
    return prefs;
  }

  test('registered instance', () {
    SharedPreferencesWindows.registerWith();
    expect(SharedPreferencesStorePlatform.instance,
        isA<SharedPreferencesWindows>());
  });

  test('getAll', () async {
    await writeTestFile(json.encode(allTestValues));
    final SharedPreferencesWindows prefs = getPreferences();

    final Map<String, Object> values = await prefs.getAll();
    expect(values, hasLength(5));
    expect(values, flutterTestValues);
  });

  test('getAllWithPrefix', () async {
    await writeTestFile(json.encode(allTestValues));
    final SharedPreferencesWindows prefs = getPreferences();

    final Map<String, Object> values = await prefs.getAllWithPrefix('prefix.');
    expect(values, hasLength(5));
    expect(values, prefixTestValues);
  });

  test('remove', () async {
    await writeTestFile('{"key1":"one","key2":2}');
    final SharedPreferencesWindows prefs = getPreferences();

    await prefs.remove('key2');

    expect(await readTestFile(), '{"key1":"one"}');
  });

  test('setValue', () async {
    await writeTestFile('{}');
    final SharedPreferencesWindows prefs = getPreferences();

    await prefs.setValue('', 'key1', 'one');
    await prefs.setValue('', 'key2', 2);

    expect(await readTestFile(), '{"key1":"one","key2":2}');
  });

  test('clear', () async {
    await writeTestFile(json.encode(flutterTestValues));
    final SharedPreferencesWindows prefs = getPreferences();

    expect(await readTestFile(), json.encode(flutterTestValues));
    await prefs.clear();
    expect(await readTestFile(), '{}');
  });

  test('clearWithPrefix', () async {
    await writeTestFile(json.encode(flutterTestValues));
    final SharedPreferencesWindows prefs = getPreferences();
    await prefs.clearWithPrefix('prefix.');
    final Map<String, Object> noValues =
        await prefs.getAllWithPrefix('prefix.');
    expect(noValues, hasLength(0));

    final Map<String, Object> values = await prefs.getAll();
    expect(values, hasLength(5));
    expect(values, flutterTestValues);
  });

  test('getAllWithNoPrefix', () async {
    await writeTestFile(json.encode(allTestValues));
    final SharedPreferencesWindows prefs = getPreferences();

    final Map<String, Object> values = await prefs.getAllWithPrefix('');
    expect(values, hasLength(15));
    expect(values, allTestValues);
  });

  test('clearWithNoPrefix', () async {
    await writeTestFile(json.encode(flutterTestValues));
    final SharedPreferencesWindows prefs = getPreferences();
    await prefs.clearWithPrefix('');
    final Map<String, Object> noValues = await prefs.getAllWithPrefix('');
    expect(noValues, hasLength(0));
  });
}

/// Fake implementation of PathProviderWindows that returns hard-coded paths,
/// allowing tests to run on any platform.
///
/// Note that this should only be used with an in-memory filesystem, as the
/// path it returns is a root path that does not actually exist on Windows.
class FakePathProviderWindows extends PathProviderPlatform
    implements PathProviderWindows {
  @override
  late VersionInfoQuerier versionInfoQuerier;

  @override
  Future<String?> getApplicationSupportPath() async => r'C:\appsupport';

  @override
  Future<String?> getTemporaryPath() async => null;

  @override
  Future<String?> getLibraryPath() async => null;

  @override
  Future<String?> getApplicationDocumentsPath() async => null;

  @override
  Future<String?> getDownloadsPath() async => null;

  @override
  Future<String> getPath(String folderID) async => '';
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:shared_preferences_windows/shared_preferences_windows.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('SharedPreferencesWindows', () {
    const Map<String, Object> kTestValues = <String, Object>{
      'flutter.String': 'hello world',
      'flutter.bool': true,
      'flutter.int': 42,
      'flutter.double': 3.14159,
      'flutter.List': <String>['foo', 'bar'],
    };

    const Map<String, Object> kTestValues2 = <String, Object>{
      'flutter.String': 'goodbye world',
      'flutter.bool': false,
      'flutter.int': 1337,
      'flutter.double': 2.71828,
      'flutter.List': <String>['baz', 'quox'],
    };

    testWidgets('reading', (WidgetTester _) async {
      final SharedPreferencesWindows preferences = SharedPreferencesWindows();
      preferences.clear();
      final Map<String, Object> values = await preferences.getAll();
      expect(values['String'], isNull);
      expect(values['bool'], isNull);
      expect(values['int'], isNull);
      expect(values['double'], isNull);
      expect(values['List'], isNull);
    });

    testWidgets('writing', (WidgetTester _) async {
      final SharedPreferencesWindows preferences = SharedPreferencesWindows();
      preferences.clear();
      await preferences.setValue(
          'String', 'flutter.String', kTestValues2['flutter.String']!);
      await preferences.setValue(
          'Bool', 'flutter.bool', kTestValues2['flutter.bool']!);
      await preferences.setValue(
          'Int', 'flutter.int', kTestValues2['flutter.int']!);
      await preferences.setValue(
          'Double', 'flutter.double', kTestValues2['flutter.double']!);
      await preferences.setValue(
          'StringList', 'flutter.List', kTestValues2['flutter.List']!);
      final Map<String, Object> values = await preferences.getAll();
      expect(values['flutter.String'], kTestValues2['flutter.String']);
      expect(values['flutter.bool'], kTestValues2['flutter.bool']);
      expect(values['flutter.int'], kTestValues2['flutter.int']);
      expect(values['flutter.double'], kTestValues2['flutter.double']);
      expect(values['flutter.List'], kTestValues2['flutter.List']);
    });

    testWidgets('removing', (WidgetTester _) async {
      final SharedPreferencesWindows preferences = SharedPreferencesWindows();
      preferences.clear();
      const String key = 'flutter.testKey';
      await preferences.setValue('String', key, kTestValues['flutter.String']!);
      await preferences.setValue('Bool', key, kTestValues['flutter.bool']!);
      await preferences.setValue('Int', key, kTestValues['flutter.int']!);
      await preferences.setValue('Double', key, kTestValues['flutter.double']!);
      await preferences.setValue(
          'StringList', key, kTestValues['flutter.List']!);
      await preferences.remove(key);
      final Map<String, Object> values = await preferences.getAll();
      expect(values[key], isNull);
    });

    testWidgets('clearing', (WidgetTester _) async {
      final SharedPreferencesWindows preferences = SharedPreferencesWindows();
      preferences.clear();
      await preferences.setValue(
          'String', 'flutter.String', kTestValues['flutter.String']!);
      await preferences.setValue(
          'Bool', 'flutter.bool', kTestValues['flutter.bool']!);
      await preferences.setValue(
          'Int', 'flutter.int', kTestValues['flutter.int']!);
      await preferences.setValue(
          'Double', 'flutter.double', kTestValues['flutter.double']!);
      await preferences.setValue(
          'StringList', 'flutter.List', kTestValues['flutter.List']!);
      await preferences.clear();
      final Map<String, Object> values = await preferences.getAll();
      expect(values['flutter.String'], null);
      expect(values['flutter.bool'], null);
      expect(values['flutter.int'], null);
      expect(values['flutter.double'], null);
      expect(values['flutter.List'], null);
    });
  });
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ignore_for_file: public_member_api_docs

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:shared_preferences_windows/shared_preferences_windows.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      title: 'SharedPreferences Demo',
      home: SharedPreferencesDemo(),
    );
  }
}

class SharedPreferencesDemo extends StatefulWidget {
  const SharedPreferencesDemo({super.key});

  @override
  SharedPreferencesDemoState createState() => SharedPreferencesDemoState();
}

class SharedPreferencesDemoState extends State<SharedPreferencesDemo> {
  final SharedPreferencesWindows prefs = SharedPreferencesWindows();
  late Future<int> _counter;

  Future<void> _incrementCounter() async {
    final Map<String, Object> values = await prefs.getAll();
    final int counter = (values['counter'] as int? ?? 0) + 1;

    setState(() {
      _counter = prefs.setValue('Int', 'counter', counter).then((bool success) {
        return counter;
      });
    });
  }

  @override
  void initState() {
    super.initState();
    _counter = prefs.getAll().then((Map<String, Object> values) {
      return values['counter'] as int? ?? 0;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SharedPreferences Demo'),
      ),
      body: Center(
          child: FutureBuilder<int>(
              future: _counter,
              builder: (BuildContext context, AsyncSnapshot<int> snapshot) {
                switch (snapshot.connectionState) {
                  case ConnectionState.none:
                  case ConnectionState.waiting:
                    return const CircularProgressIndicator();
                  case ConnectionState.active:
                  case ConnectionState.done:
                    if (snapshot.hasError) {
                      return Text('Error: ${snapshot.error}');
                    } else {
                      return Text(
                        'Button tapped ${snapshot.data} time${snapshot.data == 1 ? '' : 's'}.\n\n'
                        'This should persist across restarts.',
                      );
                    }
                }
              })),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:convert' show json;

import 'package:file/file.dart';
import 'package:file/local.dart';
import 'package:flutter/foundation.dart' show debugPrint, visibleForTesting;
import 'package:path/path.dart' as path;
import 'package:path_provider_windows/path_provider_windows.dart';
import 'package:shared_preferences_platform_interface/shared_preferences_platform_interface.dart';

/// The Windows implementation of [SharedPreferencesStorePlatform].
///
/// This class implements the `package:shared_preferences` functionality for Windows.
class SharedPreferencesWindows extends SharedPreferencesStorePlatform {
  /// Deprecated instance of [SharedPreferencesWindows].
  /// Use [SharedPreferencesStorePlatform.instance] instead.
  @Deprecated('Use `SharedPreferencesStorePlatform.instance` instead.')
  static SharedPreferencesWindows instance = SharedPreferencesWindows();

  /// Registers the Windows implementation.
  static void registerWith() {
    SharedPreferencesStorePlatform.instance = SharedPreferencesWindows();
  }

  static const String _defaultPrefix = 'flutter.';

  /// File system used to store to disk. Exposed for testing only.
  @visibleForTesting
  FileSystem fs = const LocalFileSystem();

  /// The path_provider_windows instance used to find the support directory.
  @visibleForTesting
  PathProviderWindows pathProvider = PathProviderWindows();

  /// Local copy of preferences
  Map<String, Object>? _cachedPreferences;

  /// Cached file for storing preferences.
  File? _localDataFilePath;

  /// Gets the file where the preferences are stored.
  Future<File?> _getLocalDataFile() async {
    if (_localDataFilePath != null) {
      return _localDataFilePath!;
    }
    final String? directory = await pathProvider.getApplicationSupportPath();
    if (directory == null) {
      return null;
    }
    return _localDataFilePath =
        fs.file(path.join(directory, 'shared_preferences.json'));
  }

  /// Gets the preferences from the stored file. Once read, the preferences are
  /// maintained in memory.
  Future<Map<String, Object>> _reload() async {
    Map<String, Object> preferences = <String, Object>{};
    final File? localDataFile = await _getLocalDataFile();
    if (localDataFile != null && localDataFile.existsSync()) {
      final String stringMap = localDataFile.readAsStringSync();
      if (stringMap.isNotEmpty) {
        final Object? data = json.decode(stringMap);
        if (data is Map) {
          preferences = data.cast<String, Object>();
        }
      }
    }
    _cachedPreferences = preferences;
    return preferences;
  }

  Future<Map<String, Object>> _readPreferences() async {
    return _cachedPreferences ?? await _reload();
  }

  /// Writes the cached preferences to disk. Returns [true] if the operation
  /// succeeded.
  Future<bool> _writePreferences(Map<String, Object> preferences) async {
    try {
      final File? localDataFile = await _getLocalDataFile();
      if (localDataFile == null) {
        debugPrint('Unable to determine where to write preferences.');
        return false;
      }
      if (!localDataFile.existsSync()) {
        localDataFile.createSync(recursive: true);
      }
      final String stringMap = json.encode(preferences);
      localDataFile.writeAsStringSync(stringMap);
    } catch (e) {
      debugPrint('Error saving preferences to disk: $e');
      return false;
    }
    return true;
  }

  @override
  Future<bool> clear() async {
    return clearWithPrefix(_defaultPrefix);
  }

  @override
  Future<bool> clearWithPrefix(String prefix) async {
    final Map<String, Object> preferences = await _readPreferences();
    preferences.removeWhere((String key, _) => key.startsWith(prefix));
    return _writePreferences(preferences);
  }

  @override
  Future<Map<String, Object>> getAll() async {
    return getAllWithPrefix(_defaultPrefix);
  }

  @override
  Future<Map<String, Object>> getAllWithPrefix(String prefix) async {
    final Map<String, Object> withPrefix =
        Map<String, Object>.from(await _readPreferences());
    withPrefix.removeWhere((String key, _) => !key.startsWith(prefix));
    return withPrefix;
  }

  @override
  Future<bool> remove(String key) async {
    final Map<String, Object> preferences = await _readPreferences();
    preferences.remove(key);
    return _writePreferences(preferences);
  }

  @override
  Future<bool> setValue(String valueType, String key, Object value) async {
    final Map<String, Object> preferences = await _readPreferences();
    preferences[key] = value;
    return _writePreferences(preferences);
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
import 'dart:ffi';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:path_provider_windows/path_provider_windows.dart';
import 'package:path_provider_windows/src/path_provider_windows_real.dart'
    show languageEn, encodingCP1252, encodingUnicode;

// A fake VersionInfoQuerier that just returns preset responses.
class FakeVersionInfoQuerier implements VersionInfoQuerier {
  FakeVersionInfoQuerier(
    this.responses, {
    this.language = languageEn,
    this.encoding = encodingUnicode,
  });

  final String language;
  final String encoding;
  final Map<String, String> responses;

  String? getStringValue(
    Pointer<Uint8>? versionInfo,
    String key, {
    required String language,
    required String encoding,
  }) {
    if (language == this.language && encoding == this.encoding) {
      return responses[key];
    } else {
      return null;
    }
  }
}

void main() {
  test('registered instance', () {
    PathProviderWindows.registerWith();
    expect(PathProviderPlatform.instance, isA<PathProviderWindows>());
  });

  test('getTemporaryPath', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    expect(await pathProvider.getTemporaryPath(), contains(r'C:\'));
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with no version info', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier =
        FakeVersionInfoQuerier(<String, String>{});
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, contains(r'C:\'));
    expect(path, contains(r'AppData'));
    // The last path component should be the executable name.
    expect(path, endsWith(r'flutter_tester'));
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with full version info in CP1252', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'CompanyName': 'A Company',
      'ProductName': 'Amazing App',
    }, encoding: encodingCP1252);
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, isNotNull);
    if (path != null) {
      expect(path, endsWith(r'AppData\Roaming\A Company\Amazing App'));
      expect(Directory(path).existsSync(), isTrue);
    }
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with full version info in Unicode', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'CompanyName': 'A Company',
      'ProductName': 'Amazing App',
    });
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, isNotNull);
    if (path != null) {
      expect(path, endsWith(r'AppData\Roaming\A Company\Amazing App'));
      expect(Directory(path).existsSync(), isTrue);
    }
  }, skip: !Platform.isWindows);

  test(
      'getApplicationSupportPath with full version info in Unsupported Encoding',
      () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'CompanyName': 'A Company',
      'ProductName': 'Amazing App',
    }, language: '0000', encoding: '0000');
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, contains(r'C:\'));
    expect(path, contains(r'AppData'));
    // The last path component should be the executable name.
    expect(path, endsWith(r'flutter_tester'));
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with missing company', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'ProductName': 'Amazing App',
    });
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, isNotNull);
    if (path != null) {
      expect(path, endsWith(r'AppData\Roaming\Amazing App'));
      expect(Directory(path).existsSync(), isTrue);
    }
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with problematic values', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'CompanyName': r'A <Bad> Company: Name.',
      'ProductName': r'A"/Terrible\|App?*Name',
    });
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, isNotNull);
    if (path != null) {
      expect(
          path,
          endsWith(
              r'AppData\Roaming\A _Bad_ Company_ Name\A__Terrible__App__Name'));
      expect(Directory(path).existsSync(), isTrue);
    }
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with a completely invalid company', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'CompanyName': r'..',
      'ProductName': r'Amazing App',
    });
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, isNotNull);
    if (path != null) {
      expect(path, endsWith(r'AppData\Roaming\Amazing App'));
      expect(Directory(path).existsSync(), isTrue);
    }
  }, skip: !Platform.isWindows);

  test('getApplicationSupportPath with very long app name', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    final String truncatedName = 'A' * 255;
    pathProvider.versionInfoQuerier = FakeVersionInfoQuerier(<String, String>{
      'CompanyName': 'A Company',
      'ProductName': truncatedName * 2,
    });
    final String? path = await pathProvider.getApplicationSupportPath();
    expect(path, endsWith('\\$truncatedName'));
    // The directory won't exist, since it's longer than MAXPATH, so don't check
    // that here.
  }, skip: !Platform.isWindows);

  test('getApplicationDocumentsPath', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    final String? path = await pathProvider.getApplicationDocumentsPath();
    expect(path, contains(r'C:\'));
    expect(path, contains(r'Documents'));
  }, skip: !Platform.isWindows);

  test('getDownloadsPath', () async {
    final PathProviderWindows pathProvider = PathProviderWindows();
    final String? path = await pathProvider.getDownloadsPath();
    expect(path, contains(r'C:\'));
    expect(path, contains(r'Downloads'));
  }, skip: !Platform.isWindows);
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:integration_test/integration_test_driver.dart';

Future<void> main() => integrationDriver();
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path_provider_windows/path_provider_windows.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('getTemporaryDirectory', (WidgetTester tester) async {
    final PathProviderWindows provider = PathProviderWindows();
    final String? result = await provider.getTemporaryPath();
    _verifySampleFile(result, 'temporaryDirectory');
  });

  testWidgets('getApplicationDocumentsDirectory', (WidgetTester tester) async {
    final PathProviderWindows provider = PathProviderWindows();
    final String? result = await provider.getApplicationDocumentsPath();
    _verifySampleFile(result, 'applicationDocuments');
  });

  testWidgets('getApplicationSupportDirectory', (WidgetTester tester) async {
    final PathProviderWindows provider = PathProviderWindows();
    final String? result = await provider.getApplicationSupportPath();
    _verifySampleFile(result, 'applicationSupport');
  });

  testWidgets('getDownloadsDirectory', (WidgetTester tester) async {
    final PathProviderWindows provider = PathProviderWindows();
    final String? result = await provider.getDownloadsPath();
    _verifySampleFile(result, 'downloads');
  });
}

/// Verify a file called [name] in [directoryPath] by recreating it with test
/// contents when necessary.
void _verifySampleFile(String? directoryPath, String name) {
  expect(directoryPath, isNotNull);
  if (directoryPath == null) {
    return;
  }
  final Directory directory = Directory(directoryPath);
  final File file = File('${directory.path}${Platform.pathSeparator}$name');

  if (file.existsSync()) {
    file.deleteSync();
    expect(file.existsSync(), isFalse);
  }

  file.writeAsStringSync('Hello world!');
  expect(file.readAsStringSync(), 'Hello world!');
  expect(directory.listSync(), isNotEmpty);
  file.deleteSync();
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ignore_for_file: public_member_api_docs

import 'package:flutter/material.dart';
import 'package:path_provider_windows/path_provider_windows.dart';

void main() {
  runApp(const MyApp());
}

/// Sample app
class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String? _tempDirectory = 'Unknown';
  String? _downloadsDirectory = 'Unknown';
  String? _appSupportDirectory = 'Unknown';
  String? _documentsDirectory = 'Unknown';

  @override
  void initState() {
    super.initState();
    initDirectories();
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initDirectories() async {
    String? tempDirectory;
    String? downloadsDirectory;
    String? appSupportDirectory;
    String? documentsDirectory;
    final PathProviderWindows provider = PathProviderWindows();

    try {
      tempDirectory = await provider.getTemporaryPath();
    } catch (exception) {
      tempDirectory = 'Failed to get temp directory: $exception';
    }
    try {
      downloadsDirectory = await provider.getDownloadsPath();
    } catch (exception) {
      downloadsDirectory = 'Failed to get downloads directory: $exception';
    }

    try {
      documentsDirectory = await provider.getApplicationDocumentsPath();
    } catch (exception) {
      documentsDirectory = 'Failed to get documents directory: $exception';
    }

    try {
      appSupportDirectory = await provider.getApplicationSupportPath();
    } catch (exception) {
      appSupportDirectory = 'Failed to get app support directory: $exception';
    }

    setState(() {
      _tempDirectory = tempDirectory;
      _downloadsDirectory = downloadsDirectory;
      _appSupportDirectory = appSupportDirectory;
      _documentsDirectory = documentsDirectory;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Path Provider example app'),
        ),
        body: Center(
          child: Column(
            children: <Widget>[
              Text('Temp Directory: $_tempDirectory\n'),
              Text('Documents Directory: $_documentsDirectory\n'),
              Text('Downloads Directory: $_downloadsDirectory\n'),
              Text('Application Support Directory: $_appSupportDirectory\n'),
            ],
          ),
        ),
      ),
    );
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// path_provider_windows is implemented using FFI; export a stub for platforms
// that don't support FFI (e.g., web) to avoid having transitive dependencies
// break web compilation.
export 'src/folders_stub.dart' if (dart.library.ffi) 'src/folders.dart';
export 'src/path_provider_windows_stub.dart'
    if (dart.library.ffi) 'src/path_provider_windows_real.dart';
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart' show visibleForTesting;
import 'package:path/path.dart' as path;
import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';
import 'package:win32/win32.dart';

import 'folders.dart';

/// Constant for en-US language used in VersionInfo keys.
@visibleForTesting
const String languageEn = '0409';

/// Constant for CP1252 encoding used in VersionInfo keys
@visibleForTesting
const String encodingCP1252 = '04e4';

/// Constant for Unicode encoding used in VersionInfo keys
@visibleForTesting
const String encodingUnicode = '04b0';

/// Wraps the Win32 VerQueryValue API call.
///
/// This class exists to allow injecting alternate metadata in tests without
/// building multiple custom test binaries.
@visibleForTesting
class VersionInfoQuerier {
  /// Returns the value for [key] in [versionInfo]s in section with given
  /// language and encoding, or null if there is no such entry,
  /// or if versionInfo is null.
  ///
  /// See https://docs.microsoft.com/en-us/windows/win32/menurc/versioninfo-resource
  /// for list of possible language and encoding values.
  String? getStringValue(
    Pointer<Uint8>? versionInfo,
    String key, {
    required String language,
    required String encoding,
  }) {
    assert(language.isNotEmpty);
    assert(encoding.isNotEmpty);
    if (versionInfo == null) {
      return null;
    }
    final Pointer<Utf16> keyPath =
        TEXT('\\StringFileInfo\\$language$encoding\\$key');
    final Pointer<UINT> length = calloc<UINT>();
    final Pointer<Pointer<Utf16>> valueAddress = calloc<Pointer<Utf16>>();
    try {
      if (VerQueryValue(versionInfo, keyPath, valueAddress, length) == 0) {
        return null;
      }
      return valueAddress.value.toDartString();
    } finally {
      calloc.free(keyPath);
      calloc.free(length);
      calloc.free(valueAddress);
    }
  }
}

/// The Windows implementation of [PathProviderPlatform]
///
/// This class implements the `package:path_provider` functionality for Windows.
class PathProviderWindows extends PathProviderPlatform {
  /// Registers the Windows implementation.
  static void registerWith() {
    PathProviderPlatform.instance = PathProviderWindows();
  }

  /// The object to use for performing VerQueryValue calls.
  @visibleForTesting
  VersionInfoQuerier versionInfoQuerier = VersionInfoQuerier();

  /// This is typically the same as the TMP environment variable.
  @override
  Future<String?> getTemporaryPath() async {
    final Pointer<Utf16> buffer = calloc<Uint16>(MAX_PATH + 1).cast<Utf16>();
    String path;

    try {
      final int length = GetTempPath(MAX_PATH, buffer);

      if (length == 0) {
        final int error = GetLastError();
        throw WindowsException(error);
      } else {
        path = buffer.toDartString();

        // GetTempPath adds a trailing backslash, but SHGetKnownFolderPath does
        // not. Strip off trailing backslash for consistency with other methods
        // here.
        if (path.endsWith(r'\')) {
          path = path.substring(0, path.length - 1);
        }
      }

      // Ensure that the directory exists, since GetTempPath doesn't.
      final Directory directory = Directory(path);
      if (!directory.existsSync()) {
        await directory.create(recursive: true);
      }

      return path;
    } finally {
      calloc.free(buffer);
    }
  }

  @override
  Future<String?> getApplicationSupportPath() async {
    final String? appDataRoot =
        await getPath(WindowsKnownFolder.RoamingAppData);
    if (appDataRoot == null) {
      return null;
    }
    final Directory directory = Directory(
        path.join(appDataRoot, _getApplicationSpecificSubdirectory()));
    // Ensure that the directory exists if possible, since it will on other
    // platforms. If the name is longer than MAXPATH, creating will fail, so
    // skip that step; it's up to the client to decide what to do with the path
    // in that case (e.g., using a short path).
    if (directory.path.length <= MAX_PATH) {
      if (!directory.existsSync()) {
        await directory.create(recursive: true);
      }
    }
    return directory.path;
  }

  @override
  Future<String?> getApplicationDocumentsPath() =>
      getPath(WindowsKnownFolder.Documents);

  @override
  Future<String?> getDownloadsPath() => getPath(WindowsKnownFolder.Downloads);

  /// Retrieve any known folder from Windows.
  ///
  /// folderID is a GUID that represents a specific known folder ID, drawn from
  /// [WindowsKnownFolder].
  Future<String?> getPath(String folderID) {
    final Pointer<Pointer<Utf16>> pathPtrPtr = calloc<Pointer<Utf16>>();
    final Pointer<GUID> knownFolderID = calloc<GUID>()..ref.setGUID(folderID);

    try {
      final int hr = SHGetKnownFolderPath(
        knownFolderID,
        KF_FLAG_DEFAULT,
        NULL,
        pathPtrPtr,
      );

      if (FAILED(hr)) {
        if (hr == E_INVALIDARG || hr == E_FAIL) {
          throw WindowsException(hr);
        }
        return Future<String?>.value();
      }

      final String path = pathPtrPtr.value.toDartString();
      return Future<String>.value(path);
    } finally {
      calloc.free(pathPtrPtr);
      calloc.free(knownFolderID);
    }
  }

  String? _getStringValue(Pointer<Uint8>? infoBuffer, String key) =>
      versionInfoQuerier.getStringValue(infoBuffer, key,
          language: languageEn, encoding: encodingCP1252) ??
      versionInfoQuerier.getStringValue(infoBuffer, key,
          language: languageEn, encoding: encodingUnicode);

  /// Returns the relative path string to append to the root directory returned
  /// by Win32 APIs for application storage (such as RoamingAppDir) to get a
  /// directory that is unique to the application.
  ///
  /// The convention is to use company-name\product-name\. This will use that if
  /// possible, using the data in the VERSIONINFO resource, with the following
  /// fallbacks:
  /// - If the company name isn't there, that component will be dropped.
  /// - If the product name isn't there, it will use the exe's filename (without
  ///   extension).
  String _getApplicationSpecificSubdirectory() {
    String? companyName;
    String? productName;

    final Pointer<Utf16> moduleNameBuffer = wsalloc(MAX_PATH + 1);
    final Pointer<DWORD> unused = calloc<DWORD>();
    Pointer<BYTE>? infoBuffer;
    try {
      // Get the module name.
      final int moduleNameLength =
          GetModuleFileName(0, moduleNameBuffer, MAX_PATH);
      if (moduleNameLength == 0) {
        final int error = GetLastError();
        throw WindowsException(error);
      }

      // From that, load the VERSIONINFO resource
      final int infoSize = GetFileVersionInfoSize(moduleNameBuffer, unused);
      if (infoSize != 0) {
        infoBuffer = calloc<BYTE>(infoSize);
        if (GetFileVersionInfo(moduleNameBuffer, 0, infoSize, infoBuffer) ==
            0) {
          calloc.free(infoBuffer);
          infoBuffer = null;
        }
      }
      companyName =
          _sanitizedDirectoryName(_getStringValue(infoBuffer, 'CompanyName'));
      productName =
          _sanitizedDirectoryName(_getStringValue(infoBuffer, 'ProductName'));

      // If there was no product name, use the executable name.
      productName ??=
          path.basenameWithoutExtension(moduleNameBuffer.toDartString());

      return companyName != null
          ? path.join(companyName, productName)
          : productName;
    } finally {
      calloc.free(moduleNameBuffer);
      calloc.free(unused);
      if (infoBuffer != null) {
        calloc.free(infoBuffer);
      }
    }
  }

  /// Makes [rawString] safe as a directory component. See
  /// https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions
  ///
  /// If after sanitizing the string is empty, returns null.
  String? _sanitizedDirectoryName(String? rawString) {
    if (rawString == null) {
      return null;
    }
    String sanitized = rawString
        // Replace banned characters.
        .replaceAll(RegExp(r'[<>:"/\\|?*]'), '_')
        // Remove trailing whitespace.
        .trimRight()
        // Ensure that it does not end with a '.'.
        .replaceAll(RegExp(r'[.]+$'), '');
    const int kMaxComponentLength = 255;
    if (sanitized.length > kMaxComponentLength) {
      sanitized = sanitized.substring(0, kMaxComponentLength);
    }
    return sanitized.isEmpty ? null : sanitized;
  }
}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Stub version of the actual class.
class WindowsKnownFolder {}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:path_provider_platform_interface/path_provider_platform_interface.dart';

/// A stub implementation to satisfy compilation of multi-platform packages that
/// depend on path_provider_windows. This should never actually be created.
///
/// Notably, because path_provider needs to manually register
/// path_provider_windows, anything with a transitive dependency on
/// path_provider will also depend on path_provider_windows, not just at the
/// pubspec level but the code level.
class PathProviderWindows extends PathProviderPlatform {
  /// Errors on attempted instantiation of the stub. It exists only to satisfy
  /// compile-time dependencies, and should never actually be created.
  PathProviderWindows() : assert(false);

  /// Registers the Windows implementation.
  static void registerWith() {
    PathProviderPlatform.instance = PathProviderWindows();
  }

  /// Stub; see comment on VersionInfoQuerier.
  VersionInfoQuerier versionInfoQuerier = VersionInfoQuerier();

  /// Match PathProviderWindows so that the analyzer won't report invalid
  /// overrides if tests provide fake PathProviderWindows implementations.
  Future<String> getPath(String folderID) async => '';
}

/// Stub to satisfy the analyzer, which doesn't seem to handle conditional
/// exports correctly.
class VersionInfoQuerier {}
```

```dart
// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'package:win32/win32.dart';

// ignore_for_file: non_constant_identifier_names

// ignore: avoid_classes_with_only_static_members
/// A class containing the GUID references for each of the documented Windows
/// known folders. A property of this class may be passed to the `getPath`
/// method in the [PathProvidersWindows] class to retrieve a known folder from
/// Windows.
class WindowsKnownFolder {
  /// The file system directory that is used to store administrative tools for
  /// an individual user. The MMC will save customized consoles to this
  /// directory, and it will roam with the user.
  static String get AdminTools => FOLDERID_AdminTools;

  /// The file system directory that acts as a staging area for files waiting to
  /// be written to a CD. A typical path is C:\Documents and
  /// Settings\username\Local Settings\Application Data\Microsoft\CD Burning.
  static String get CDBurning => FOLDERID_CDBurning;

  /// The file system directory that contains administrative tools for all users
  /// of the computer.
  static String get CommonAdminTools => FOLDERID_CommonAdminTools;

  /// The file system directory that contains the directories for the common
  /// program groups that appear on the Start menu for all users. A typical path
  /// is C:\Documents and Settings\All Users\Start Menu\Programs.
  static String get CommonPrograms => FOLDERID_CommonPrograms;

  /// The file system directory that contains the programs and folders that
  /// appear on the Start menu for all users. A typical path is C:\Documents and
  /// Settings\All Users\Start Menu.
  static String get CommonStartMenu => FOLDERID_CommonStartMenu;

  /// The file system directory that contains the programs that appear in the
  /// Startup folder for all users. A typical path is C:\Documents and
  /// Settings\All Users\Start Menu\Programs\Startup.
  static String get CommonStartup => FOLDERID_CommonStartup;

  /// The file system directory that contains the templates that are available
  /// to all users. A typical path is C:\Documents and Settings\All
  /// Users\Templates.
  static String get CommonTemplates => FOLDERID_CommonTemplates;

  /// The virtual folder that represents My Computer, containing everything on
  /// the local computer: storage devices, printers, and Control Panel. The
  /// folder can also contain mapped network drives.
  static String get ComputerFolder => FOLDERID_ComputerFolder;

  /// The virtual folder that represents Network Connections, that contains
  /// network and dial-up connections.
  static String get ConnectionsFolder => FOLDERID_ConnectionsFolder;

  /// The virtual folder that contains icons for the Control Panel applications.
  static String get ControlPanelFolder => FOLDERID_ControlPanelFolder;

  /// The file system directory that serves as a common repository for Internet
  /// cookies. A typical path is C:\Documents and Settings\username\Cookies.
  static String get Cookies => FOLDERID_Cookies;

  /// The virtual folder that represents the Windows desktop, the root of the
  /// namespace.
  static String get Desktop => FOLDERID_Desktop;

  /// The virtual folder that represents the My Documents desktop item.
  static String get Documents => FOLDERID_Documents;

  /// The file system directory that serves as a repository for Internet
  /// downloads.
  static String get Downloads => FOLDERID_Downloads;

  /// The file system directory that serves as a common repository for the
  /// user's favorite items. A typical path is C:\Documents and
  /// Settings\username\Favorites.
  static String get Favorites => FOLDERID_Favorites;

  /// A virtual folder that contains fonts. A typical path is C:\Windows\Fonts.
  static String get Fonts => FOLDERID_Fonts;

  /// The file system directory that serves as a common repository for Internet
  /// history items.
  static String get History => FOLDERID_History;

  /// The file system directory that serves as a common repository for temporary
  /// Internet files. A typical path is C:\Documents and Settings\username\Local
  /// Settings\Temporary Internet Files.
  static String get InternetCache => FOLDERID_InternetCache;

  /// A virtual folder for Internet Explorer.
  static String get InternetFolder => FOLDERID_InternetFolder;

  /// The file system directory that serves as a data repository for local
  /// (nonroaming) applications. A typical path is C:\Documents and
  /// Settings\username\Local Settings\Application Data.
  static String get LocalAppData => FOLDERID_LocalAppData;

  /// The file system directory that serves as a common repository for music
  /// files. A typical path is C:\Documents and Settings\User\My Documents\My
  /// Music.
  static String get Music => FOLDERID_Music;

  /// A file system directory that contains the link objects that may exist in
  /// the My Network Places virtual folder. A typical path is C:\Documents and
  /// Settings\username\NetHood.
  static String get NetHood => FOLDERID_NetHood;

  /// The folder that represents other computers in your workgroup.
  static String get NetworkFolder => FOLDERID_NetworkFolder;

  /// The file system directory that serves as a common repository for image
  /// files. A typical path is C:\Documents and Settings\username\My
  /// Documents\My Pictures.
  static String get Pictures => FOLDERID_Pictures;

  /// The file system directory that contains the link objects that can exist in
  /// the Printers virtual folder. A typical path is C:\Documents and
  /// Settings\username\PrintHood.
  static String get PrintHood => FOLDERID_PrintHood;

  /// The virtual folder that contains installed printers.
  static String get PrintersFolder => FOLDERID_PrintersFolder;

  /// The user's profile folder. A typical path is C:\Users\username.
  /// Applications should not create files or folders at this level.
  static String get Profile => FOLDERID_Profile;

  /// The file system directory that contains application data for all users. A
  /// typical path is C:\Documents and Settings\All Users\Application Data. This
  /// folder is used for application data that is not user specific. For
  /// example, an application can store a spell-check dictionary, a database of
  /// clip art, or a log file in the CSIDL_COMMON_APPDATA folder. This
  /// information will not roam and is available to anyone using the computer.
  static String get ProgramData => FOLDERID_ProgramData;

  /// The Program Files folder. A typical path is C:\Program Files.
  static String get ProgramFiles => FOLDERID_ProgramFiles;

  /// The common Program Files folder. A typical path is C:\Program
  /// Files\Common.
  static String get ProgramFilesCommon => FOLDERID_ProgramFilesCommon;

  /// On 64-bit systems, a link to the common Program Files folder. A typical path is
  /// C:\Program Files\Common Files.
  static String get ProgramFilesCommonX64 => FOLDERID_ProgramFilesCommonX64;

  /// On 64-bit systems, a link to the 32-bit common Program Files folder. A
  /// typical path is C:\Program Files (x86)\Common Files. On 32-bit systems, a
  /// link to the Common Program Files folder.
  static String get ProgramFilesCommonX86 => FOLDERID_ProgramFilesCommonX86;

  /// On 64-bit systems, a link to the Program Files folder. A typical path is
  /// C:\Program Files.
  static String get ProgramFilesX64 => FOLDERID_ProgramFilesX64;

  /// On 64-bit systems, a link to the 32-bit Program Files folder. A typical
  /// path is C:\Program Files (x86). On 32-bit systems, a link to the Common
  /// Program Files folder.
  static String get ProgramFilesX86 => FOLDERID_ProgramFilesX86;

  /// The file system directory that contains the user's program groups (which
  /// are themselves file system directories).
  static String get Programs => FOLDERID_Programs;

  /// The file system directory that contains files and folders that appear on
  /// the desktop for all users. A typical path is C:\Documents and Settings\All
  /// Users\Desktop.
  static String get PublicDesktop => FOLDERID_PublicDesktop;

  /// The file system directory that contains documents that are common to all
  /// users. A typical path is C:\Documents and Settings\All Users\Documents.
  static String get PublicDocuments => FOLDERID_PublicDocuments;

  /// The file system directory that serves as a repository for music files
  /// common to all users. A typical path is C:\Documents and Settings\All
  /// Users\Documents\My Music.
  static String get PublicMusic => FOLDERID_PublicMusic;

  /// The file system directory that serves as a repository for image files
  /// common to all users. A typical path is C:\Documents and Settings\All
  /// Users\Documents\My Pictures.
  static String get PublicPictures => FOLDERID_PublicPictures;

  /// The file system directory that serves as a repository for video files
  /// common to all users. A typical path is C:\Documents and Settings\All
  /// Users\Documents\My Videos.
  static String get PublicVideos => FOLDERID_PublicVideos;

  /// The file system directory that contains shortcuts to the user's most
  /// recently used documents. A typical path is C:\Documents and
  /// Settings\username\My Recent Documents.
  static String get Recent => FOLDERID_Recent;

  /// The virtual folder that contains the objects in the user's Recycle Bin.
  static String get RecycleBinFolder => FOLDERID_RecycleBinFolder;

  /// The file system directory that contains resource data. A typical path is
  /// C:\Windows\Resources.
  static String get ResourceDir => FOLDERID_ResourceDir;

  /// The file system directory that serves as a common repository for
  /// application-specific data. A typical path is C:\Documents and
  /// Settings\username\Application Data.
  static String get RoamingAppData => FOLDERID_RoamingAppData;

  /// The file system directory that contains Send To menu items. A typical path
  /// is C:\Documents and Settings\username\SendTo.
  static String get SendTo => FOLDERID_SendTo;

  /// The file system directory that contains Start menu items. A typical path
  /// is C:\Documents and Settings\username\Start Menu.
  static String get StartMenu => FOLDERID_StartMenu;

  /// The file system directory that corresponds to the user's Startup program
  /// group. The system starts these programs whenever the associated user logs
  /// on. A typical path is C:\Documents and Settings\username\Start
  /// Menu\Programs\Startup.
  static String get Startup => FOLDERID_Startup;

  /// The Windows System folder. A typical path is C:\Windows\System32.
  static String get System => FOLDERID_System;

  /// The 32-bit Windows System folder. On 32-bit systems, this is typically
  /// C:\Windows\system32. On 64-bit systems, this is typically
  /// C:\Windows\syswow64.
  static String get SystemX86 => FOLDERID_SystemX86;

  /// The file system directory that serves as a common repository for document
  /// templates. A typical path is C:\Documents and Settings\username\Templates.
  static String get Templates => FOLDERID_Templates;

  /// The file system directory that serves as a common repository for video
  /// files. A typical path is C:\Documents and Settings\username\My
  /// Documents\My Videos.
  static String get Videos => FOLDERID_Videos;

  /// The Windows directory or SYSROOT. This corresponds to the %windir% or
  /// %SYSTEMROOT% environment variables. A typical path is C:\Windows.
  static String get Windows => FOLDERID_Windows;
}
```

