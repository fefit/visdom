macro_rules! cfg_feat_text {
	($($item:item)*) => {
    $(
      #[cfg(feature = "text")]
      $item
    )*
  };
}
