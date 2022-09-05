test_that("cast Robj of externalPointer to Mystruct as reference to MyStruct", {
  
  
  mystruct  = helloextendr:::MyStruct$new()
  
  #restore_from_robj must take Robj, not MyStruct as input
  mystruct2 = helloextendr:::MyStruct$restore_from_robj(mystruct)
  
})
