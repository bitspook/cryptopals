(setq obs-lang "obs")

(defvar org-babel-header-args:obs '((:module . :any)))

(defun obs--org-src-html-block-advice (oldfun src-block contents info)
  "Turn org-src-block to an observable notebook cell.

Usage:
    #+begin_src obs :module main
    myVar = \"myVal\"
    #+end_src
"
  (let* ((old-ret (funcall oldfun src-block contents info))
         (lang (org-element-property :language src-block))
         (module (or (org-export-read-attribute :attr_obs src-block :module) "main"))
         (id-attr (or (org-export-read-attribute :attr_obcell src-block :id) (make-temp-name "ob-cell"))))
    (if (string= lang obs-lang)
        (concat
         "<ob-cell module=" module
         " id="
         (format "\"%s\"" id-attr)
         ">"
         old-ret
         "</ob-cell>")
      old-ret)))

(add-to-list 'org-src-lang-modes '(obs-lang . js))

(advice-add 'org-html-src-block :around #'obs--org-src-html-block-advice)
