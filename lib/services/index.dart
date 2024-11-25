import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class MyData extends ChangeNotifier {
  String data = "Hello, Provider!";

  void updateData(String newData) {
    data = newData;
    notifyListeners();
  }
}
