;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((typescript-tsx-mode . ((prettier-format-on-save-mode . t)))
 (typescript-mode . ((prettier-format-on-save-mode . t)))
 (scss-mode . ((prettier-format-on-save-mode . t)))
 (org-mode . ((org-publish-project-alist . (("cryptopals"
					                                   :base-directory "./"
                                             :recursive nil
					                                   :base-exteinsion "org"
					                                   :publishing-directory "./web"
					                                   :publishing-function org-html-publish-to-html
					                                   :auto-preamble t)))
              (eval (lambda ()
                      (add-hook 'after-save-hook #'org-publish-current-project nil t)
                      (load-file "./observab.el"))))))
