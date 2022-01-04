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
         (id-attr (or (org-export-read-attribute :attr_obcell src-block :id) (make-temp-name "ob-cell"))))
    (if obcell
        (concat
         "<div class=\"ob-cell\""
         (if module (format "data-module=\"%s\" " module))
         (if id-attr (format "id=\"%s\" " id-attr))
         ">"
         old-ret
         "</div>")
      old-ret)))

(advice-add 'org-html-src-block :around #'orgservable--org-src-html-block-advice)
