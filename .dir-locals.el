;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((org-mode . ((org-publish-project-alist . (("cryptopals"
					                                   :base-directory "./"
					                                   :base-exteinsion "org"
					                                   :publishing-directory "./web"
					                                   :publishing-function org-html-publish-to-html
					                                   :auto-preamble t)))
              (eval (lambda ()
                      (add-hook 'after-save-hook #'org-publish-current-project nil t)
                      (load-file "./orgservable.el"))))))
