# JCParser

A Rust-based command-line tool for interacting with CSV files using a simple query language. Perform data retrieval and modification operations.

## Features

- 🔍 **Query CSV files** using simple query syntax
- 📝 **Modify CSV data** with set operations
- ⚡ **Fast performance** built with Rust
- 🛠️ **Simple CLI interface** for easy integration into workflows

## Installation

### From Source

```bash
https://github.com/tawfiqAK17/JCParser.git
cd JCParser
cargo build --release
```

The binary will be available at `target/release/JCParser`.

## Usage

```bash
JCParser [FILE_TYPE] [OPTIONS] <FILE_PATH>
```

### Commands

#### get - Query Data

Retrieve data from CSV files using get command.

```bash
# Get all data
get *

# Get specific columns
get $field_name1 $field_name2

# Get with conditions
get * where $field_name [COMPARISON OPERATOR](#comparison-operators) [VALUE]

# Call a query function
get * [FUNCTION NAME] [FUNCTION PARAMETER]
```

check [Comparison operators](#comparison-operators), [Functions](#functions) and [Values](#values).

#### set - Modify Data

Update CSV data using set command.

```bash
# Update specific rows
set $field_name = [VALUE | MODIFICATION]
# Update multiple columns
set $field_name1 = [VALUE | MODIFICATION] $field_name2 = [VALUE | MODIFICATION]
# Update columns that satisfied a specific condition
set $field_name1 = [VALUE | MODIFICATION] $field_name2 = [VALUE | MODIFICATION] where [CONDITION]
```

check [Modification](#modification) and [Values](#values).

#### insert-row - Modify Data

Update CSV data using insert-row command.

```bash
# insert a single value 
# the other values will be set to "" (None)
insert-row $field_name = [VALUE | MODIFICATION]
# insert multiple values 
insert-row $field_name1 = [VALUE | MODIFICATION] $field_name2 = [VALUE | MODIFICATION]
```

#### insert-column - Modify Data

Update CSV data using insert-column command.

```bash
# insert a new column 
insert-column $field_name = [VALUE | MODIFICATION]
# insert multiple columns
insert-column $field_name1 = [VALUE | MODIFICATION] $field_name2 = [VALUE | MODIFICATION]
# insert a column for only the rows that satisfies a condition 
# the value of the new column for other rows will be set to "" (None)
insert-column $field_name1 = [VALUE | MODIFICATION] $field_name2 = [VALUE | MODIFICATION] where [CONDITION]
```

#### Values:

A vlaue can be a field name(ex: $age), number(ex: 25), string(ex: "bob") or a list(ex:[1, 2, 3] or ["foo", "bar"])

#### Comparison operators:

| operator    | description                                  | example                                    |
| :---------- | :------------------------------------------- | :----------------------------------------- |
| ==          | equal                                        | where $age == 25                           |
| !=          | not equal                                    | where $age != 25                           |
| <           | less than                                    | where $age < 25                            |
| >           | greater than                                 | where $age > 25                            |
| <=          | less than or equal                           | where $age <= 25                           |
| >=          | greater than or equal                        | where $age >= 25                           |
| between     | between                                      | where $age between 25 and 35               |
| is          | equal strings                                | where $name is "John"                      |
| isnot       | not equal strings                            | where $status isnot "active"               |
| contains    | if a string contains an other one            | where $email contains "@gmail"             |
| starts-with | if a string starts with an other one         | where $name starts-with "A"                |
| ends-with   | if a string ends with an other one           | where $file ends-with ".csv"               |
| in          | if the field name value is in the given list | where $department in ["IT", "Engineering"] |

#### Functions:

| name   | parameter  | description                   |
| :----- | :--------- | :---------------------------- |
| sort   | field name | sort in ascci order           |
| rsort  | field name | sort in reverse ascci order   |
| nsort  | field name | sort in numeric order         |
| rnsort | field name | sort in reverse numeric order |
| tail   | number     | get the last n row            |
| head   | number     | get the first n row           |

#### Modification

| modifier | description                 | example                                    |
| :------- | :-------------------------- | :----------------------------------------- |
| +        | addition                    | \$salary = \$salary + $bonus               |
| -        | subtraction                 | \$total = \$total - \$discount             |
| \*       | multiplication              | \$total = \$price \* \$quantity            |
| /        | division                    | \$price = \$total / \$count                |
| %        | modulo (remainder)          | \$id = \$id % 2                            |
| ^        | power (exponentiation)      | \$result = \$base ^ $exponent              |
| \|\|     | concatenate strings         | \$full_name = \$first_name \|\| $last_name |
| to-upper | convert string to uppercase | \$name = \$name to-upper                   |
| to-lower | convert string to lowercase | \$name = \$name to-lower                   |

## Examples

### Sample CSV File (employees.csv)

```csv
id,name,age,department,salary
1,Alice,28,Engineering,75000
2,Bob,32,Marketing,65000
3,Charlie,25,Engineering,70000
4,Diana,29,Sales,68000
```

### Example Queries

```bash
# Get all employees
get *

# Get engineers only
get $department where $department is "Engineering"

# Get high earners
get * where $salary >= 70000

# Update salary
set where $name is "Alice" to $salary = 80000

# Update department for young employees
set where $age < 27 to $department = "Tech"
```

## Options

- `-csv` or `-json` - the type of the file
- `-s<CHAR>` - CSV delimiter character [default: ,]

## Error Handling

The tool provides clear error messages for:

- Invalid syntax
- File not found or permission errors
- Data type mismatches
- Malformed CSV files

## Roadmap

- [x] **INSERT operations** - Add new rows to CSV files
- [ ] **DELETE operations** - Remove rows from CSV files
- [ ] **Aggregation functions** - SUM, COUNT, AVG, etc.
- [ ] **JSON file support** - Extend functionality to JSON data

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Note**: This tool is currently in active development. The API and command syntax may change in future versions.
