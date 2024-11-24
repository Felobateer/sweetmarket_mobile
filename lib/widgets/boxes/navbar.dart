import 'package:flutter/material.dart';

class Navbar extends StatelessWidget implements PreferredSizeWidget {
  final String text;
  const Navbar({required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return AppBar(
      title: Text(text),
      iconTheme: IconThemeData(color: Colors.white),
      backgroundColor: const Color(0xFFD97706),
      titleTextStyle: const TextStyle(
        fontSize: 24,
        fontWeight: FontWeight.bold,
        color: Colors.white,
        fontFamily: 'Roboto',
      ),
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(kToolbarHeight);
}
