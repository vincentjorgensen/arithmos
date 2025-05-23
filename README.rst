=========
 arithmos
=========

.. image:: https://img.shields.io/badge/License-GPL%203.0-green.svg
   :target: https://www.gnu.org/licenses/gpl-3.0.en.html
   :alt: License: GPL-3.0

Rust library for converting Arabic numerals (1,2,3...) to Ancient Greek
numerals (α', β', γ' ...)

Example usage
=============

.. code-block:: rust

   use arithmos::GreekNumeral;

   fn main() {
      let num = GreekNumeral::new(616).unwrap();
      println!("{}", num); // ΧΙϜ'
      assert_eq!("ΧΙϜ'".parse().unwrap(), num);
   }

License
=======

`GPL-3.0`__

__ main/LICENSE

Contribution
------------

Any contribution intentionally submitted for inclusion in this project shall be
licensed as above without additional terms or conditions.
