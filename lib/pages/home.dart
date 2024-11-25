import 'package:flutter/material.dart';
import 'package:sweetmarket/widgets/boxes/appBar.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: Appbar(
      left: Text('left'),
      center: Text('center'),
      right: Text('right'),
    ));
  }
}
