import 'package:flutter/material.dart';

class customsubText extends StatelessWidget {
  final String text;
  const customsubText({required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: TextStyle(
        fontSize: 16,
        fontFamily: 'Poppins',
        color: Colors.grey,
        fontWeight: FontWeight.w400,
      ),
    );
  }
}
