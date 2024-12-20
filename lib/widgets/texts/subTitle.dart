import 'package:flutter/material.dart';

class customsubTitle extends StatelessWidget {
  final String text;
  final Color color;
  const customsubTitle({required this.color, required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      textAlign: TextAlign.center,
      style: TextStyle(
        fontSize: 22,
        fontFamily: 'Poppins',
        color: color,
        fontWeight: FontWeight.bold,
        height: 0.9,
      ),
    );
  }
}
