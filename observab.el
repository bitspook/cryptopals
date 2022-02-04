(defun orgservable--org-src-html-block-advice (oldfun src-block contents info)
  "Turn org-src-block to an observable notebook cell.

Usage:
    #+OBCELL: :module a-module
    #+begin_src js
    myVar = \"myVal\"
    #+end_src
"
  (let* ((old-ret (funcall oldfun src-block contents info))
         (obcell (org-export-read-attribute :attr_obcell src-block))
         (module (org-export-read-attribute :attr_obcell src-block :module))
         (module-name (or module "main"))
         (id-attr (or (org-export-read-attribute :attr_obcell src-block :id) (make-temp-name "ob-cell"))))
    (if obcell
        (concat
         "<ob-cell module=" module-name
         " id="
         (format "\"%s\"" id-attr)
         ">"
         old-ret
         "</ob-cell>")
      old-ret)))

(add-to-list 'org-src-lang-modes '("obs" . js))

(advice-add 'org-html-src-block :around #'orgservable--org-src-html-block-advice)
