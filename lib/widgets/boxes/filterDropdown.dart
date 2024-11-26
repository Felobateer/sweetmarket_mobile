import 'package:flutter/material.dart';
import 'package:sweetmarket/services/models/filters.dart';

class Filterdropdown extends StatefulWidget {
  final Filter filter;

  const Filterdropdown({
    required this.filter,
    super.key,
  });

  @override
  _FilterdropdownState createState() => _FilterdropdownState();
}

class _FilterdropdownState extends State<Filterdropdown> {
  String? selectedFilter;

  @override
  Widget build(BuildContext context) {
    return DropdownButton<String>(
      value: selectedFilter,
      hint: Text(widget.filter.name), // Dropdown hint text
      onChanged: (String? newValue) {
        setState(() {
          selectedFilter = newValue; // Update selected filter
        });
      },
      items:
          widget.filter.filters.map<DropdownMenuItem<String>>((String filter) {
        return DropdownMenuItem<String>(
          value: filter,
          child: Text(filter), // Display each filter as a dropdown item
        );
      }).toList(),
    );
  }
}
