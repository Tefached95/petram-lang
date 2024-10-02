(lang dune 3.16)

(name petram)

(generate_opam_files true)

(source
 (github Tefached95/petram-lang))

(authors "Tefached95")

(maintainers "Tefached95")

(license MIT)

(documentation https://url/to/documentation)

(package
 (name petram)
 (synopsis "Chiseled for clarity")
 (description "Petram is a programming language designed for clarity, simplicity, and expressiveness. It has verbosity as a feature, and several functional influences.")
 (depends ocaml dune)
 (tags
  (language functional)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/reference/dune-project/index.html
