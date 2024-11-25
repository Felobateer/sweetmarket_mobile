import 'package:flutter/material.dart';

class Appbar extends StatelessWidget implements PreferredSizeWidget {
  final Widget? left;
  final Widget? right;
  final double height; // Add a height parameter

  const Appbar({
    this.left,
    this.right,
    this.height = kToolbarHeight, // Default height if none is provided
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return AppBar(
      backgroundColor: const Color(0xFFD97706),
      iconTheme: IconThemeData(color: Colors.white),
      titleTextStyle: const TextStyle(
        fontSize: 24,
        fontWeight: FontWeight.bold,
        color: Colors.white,
        fontFamily: 'Roboto',
      ),
      leading: left, // Custom left widget (e.g., logo)
      actions: [right ?? Container()], // Custom right widget (e.g., buttons)
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(height); // Set custom height
}
