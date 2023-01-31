semver-rs 0.1.0
=============

Synopsis
--------

    CREATE EXTENSION semver-rs;

    SELECT '1.2.1'::semver;
     semver
    --------
     1.2.1

    SELECT '1.2.0'::semver > '1.2.0-b1'::semver;
     ?column?
    ----------
     t

Description
-----------

This library contains a single PostgreSQL extension, a semantic version data
type called `semver`. It's an implementation of the version number format
specified by the
[Semantic Versioning 2.0.0 Specification](https://semver.org/spec/v2.0.0.html).

The salient points of [the spec](https://semver.org/), for the purposes of
a data type and comparison operators, are:

1. A normal version number MUST take the form X.Y.Z where X, Y, and Z are
   non-negative integers, and MUST NOT contain leading zeroes. X is the major
   version, Y is the minor version, and Z is the patch version. Each element
   MUST increase numerically. For instance: `1.9.0 < 1.10.0 < 1.11.0`.

2. A pre-release version MAY be denoted by appending a hyphen and a series
   of dot separated identifiers immediately following the patch version.
   Identifiers MUST comprise only ASCII alphanumerics and hyphen [0-9A-Za-z-].
   Identifiers MUST NOT be empty. Numeric identifiers MUST NOT include leading
   zeroes. Pre-release versions have a lower precedence than the associated
   normal version. A pre-release version indicates that the version is
   unstable and might not satisfy the intended compatibility requirements
   as denoted by its associated normal version. Examples:
   `1.0.0-alpha, 1.0.0-alpha.1, 1.0.0-0.3.7, 1.0.0-x.7.z.92`.

3. Build metadata MAY be denoted by appending a plus sign and a series of
   dot separated identifiers immediately following the patch or pre-release
   version. Identifiers MUST comprise only ASCII alphanumerics and hyphen
   [0-9A-Za-z-]. Identifiers MUST NOT be empty. Build metadata SHOULD be
   ignored when determining version precedence. Thus two versions that differ
   only in the build metadata, have the same precedence. Examples:
   `1.0.0-alpha+001, 1.0.0+20130313144700, 1.0.0-beta+exp.sha.5114f85`.

4. Precedence refers to how versions are compared to each other when ordered.
   Precedence MUST be calculated by separating the version into major, minor,
   patch and pre-release identifiers in that order (Build metadata does not
   figure into precedence). Precedence is determined by the first difference
   when comparing each of these identifiers from left to right as follows:
   Major, minor, and patch versions are always compared numerically. Example:
   `1.0.0 < 2.0.0 < 2.1.0 < 2.1.1`. When major, minor, and patch are equal, a
   pre-release version has lower precedence than a normal version. Example:
   `1.0.0-alpha < 1.0.0`. Precedence for two pre-release versions with the same
   major, minor, and patch version MUST be determined by comparing each dot
   separated identifier from left to right until a difference is found as
   follows: identifiers consisting of only digits are compared numerically and
   identifiers with letters or hyphens are compared lexically in ASCII sort
   order. Numeric identifiers always have lower precedence than non-numeric
   identifiers. A larger set of pre-release fields has a higher precedence than
   a smaller set, if all of the preceding identifiers are equal. Example:
   `1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 <
   1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0`.

Usage
-----

Add the extension to a database:

    CREATE EXTENSION semver-rs;

Now, use it like any other data type. Here's an example in a table:

    CREATE TABLE extensions (
        name        TEXT,
        version     SEMVER,
        description TEXT,
        PRIMARY KEY (name, version)
    );

The type can be in indexed using btree or hash indexes:

    CREATE INDEX idx_extension_version ON extensions(version);
    CREATE INDEX hdx_extension_version ON extensions USING hash (version);

Hash indexes aren't worth much, but the functionality is there to support hash
aggregates in query optimizations.

And some sample usage:

    INSERT INTO extensions
    VALUES ('pgtap', '0.35.0',    'PostgreSQL unit testing'),
           ('pgtap', '0.35.0-b1', 'PostgreSQL unit testing.'),
           ('pair',  '0.1.0',     'Key/value pair data type'),
           ('PostGIS', '1.5.0',   'Gelocation data types');

    SELECT * FROM extensions WHERE VERSION = '1.5.0';
      name   │ version │      description
    ---------+---------+-----------------------
     PostGIS │ 1.5.0   │ Gelocation data types

    SELECT * FROM extensions WHERE VERSION < '0.35.0';
     name  │ version   │       description
    -------+-----------+--------------------------
     pgtap │ 0.35.0-b1 │ PostgreSQL unit testing.
     pair  │ 0.1.0     │ Key/value pair data type

Note that "0.35.0-b1" is less than "0.35.0", as required by the specification.
Use `ORDER BY` to get more of a feel for semantic version ordering rules:

    SELECT version FROM extensions ORDER BY version;
     version
    -----------
     0.1.0
     0.35.0-b1
     0.35.0
     1.5.0

    SELECT version FROM extensions ORDER BY version DESC;
     version
    -----------
     1.5.0
     0.35.0
     0.35.0-b1
     0.1.0

Interface
---------

### Operators ###

 Operator | Description                               | Example                             | Result
----------|-------------------------------------------|-------------------------------------|--------
 `=`      | Are semvers equivalent                    | '1.2.0'semver = '1.2.00'::semver    | `t`
 `<>`     | Are semvers different                     | '1.2.0'semver <> '1.2.00'::semver   | `f`
 `<`      | Is semver less than right semver          | '3.4.0-b1'semver < '3.4.0'::semver  | `t`
 `<=`     | Is semver less than or equal to semver    | '3.4.0-b1'semver <= '3.4.0'::semver | `t`
 `>`      | Is semver greater than right semver       | '3.4.0-b1'semver > '3.4.0'::semver  | `f`
 `>=`     | Is semver greater than or equal to semver | '3.4.0-b1'semver >= '3.4.0'::semver | `f`

### Range Type ###

The extension includes the `semverrange` type, which
simply builds on the
[range type](https://www.postgresql.org/docs/current/static/rangetypes.html)
support on PostgreSQL 9.2 and higher. This allows for easy specification of
ranges of semantic versions. Some examples:

 Range                 | Description
-----------------------|-----------------------------------
 `['1.0.0', '2.0.0']`  | 1.0.0 inclusive - 2.0.0 inclusive
 `['1.0.0', '2.0.0')`  | 1.0.0 inclusive - 2.0.0 exclusive
 `('1.0.0', '2.0.0')`  | 1.0.0 exclusive - 2.0.0 exclusive
 `['1.0.0',]`.         | 1.0.0 inclusive - infinity

The cool thing is that you can use any of the
[range operators](https://www.postgresql.org/docs/current/static/functions-range.html),
including the "contains" operators: For example, to see if `1.0.5` falls falls
within the range `1.0.0` - `2.0.0` exclusive, run a query like this:

    SELECT '1.0.5'::semver <@ '[1.0.0, 2.0.0)'::semverrange;
     ?column?
    ----------
     t

The `semverrange` constructor will build the same range,

    SELECT semverrange('1.0.0', '2.0.0') @> '2.0.0'::semver;
     ?column?
    ----------
     f

    SELECT semverrange('1.0.0', '2.0.0') @> '1.9999.9999'::semver;
     ?column?
    ----------
     t

Pass the optional third argument to determine the bounds inclusiveness:

    SELECT semverrange('1.0.0', '2.0.0', '[]') @> '2.0.0'::semver;
     ?column?
    ----------
     t

It works for unlimited bound, as well. For example, this query ensure that
a semver is greater than or equal `1.0.0`:

    SELECT '1000.0.0'::semver <@ '[1.0.0,]'::semverrange;
     ?column?
    ----------
     t

If you need to omit some values, you can use an array of semverrange values.
For example, say you want to check require a version greater than `1.0.0` and
less than `2.0.0`, but versions `1.2.3` and `1.4.5` have such serious bugs that
you don't want to include them. We create three ranges that use exclusive
bounds to omit those versions, like so:

    '{"(1.0.0,1.2.3)", "(1.2.3,1.4.5)", "(1.4.5,2.0.0)"}'::semverrange[]

Here's an sample how to query such an array of semverranges.

    SELECT version, version <@ ANY(
        '{"(1.0.0,1.2.3)", "(1.2.3,1.4.5)", "(1.4.5,2.0.0)"}'::semverrange[]
    ) AS valid FROM (VALUES
        ('1.0.0'::semver), ('1.0.1'), ('1.2.3'), ('1.2.4'), ('1.4.4'), ('1.4.5'),
        ('1.7.0'), ('2.0.0')
    ) AS v(version)
     version | valid
    ---------+-------
     1.0.0   | f
     1.0.1   | t
     1.2.3   | f
     1.2.4   | t
     1.4.4   | t
     1.4.5   | f
     1.7.0   | t
     2.0.0   | f

Support
-------

This library is stored in an open
[GitHub repository](https://github.com/fabmation-gmbh/pg_semver-rs). Feel free to fork
and contribute! Please file bug reports via
[GitHub Issues](https://github.com/fabmation-gmbh/pg_semver-rs/issues/).


Copyright and License
---------------------

MIT License

Copyright (c) 2023 FABMation GmbH

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.



_License of the [semver](https://github.com/theory/pg-semver) extension this documentation is based on:_

Copyright (c) 2010-2022 The pg-semver Maintainers: David E. Wheeler, Sam
Vilain, Tom Davis, and Xavier Caron.

This module is free software; you can redistribute it and/or modify it under
the [PostgreSQL License](https://www.opensource.org/licenses/postgresql).

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose, without fee, and without a written agreement is
hereby granted, provided that the above copyright notice and this paragraph
and the following two paragraphs appear in all copies.

In no event shall The pg-semver Maintainers be liable to any party for direct,
indirect, special, incidental, or consequential damages, including lost
profits, arising out of the use of this software and its documentation, even
if The pg-semver Maintainers have been advised of the possibility of such
damage.

The pg-semver Maintainers specifically disclaim any warranties, including, but
not limited to, the implied warranties of merchantability and fitness for a
particular purpose. The software provided hereunder is on an "as is" basis,
and The pg-semver Maintainers no obligations to provide maintenance, support,
updates, enhancements, or modifications.
