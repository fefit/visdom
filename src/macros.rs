macro_rules! cfg_feat_text {
	($($item:item)*) => {
    $(
      #[cfg(feature = "text")]
      $item
    )*
  };
}

macro_rules! cfg_feat_insertion {
	($($item:item)*) => {
    $(
      #[cfg(feature = "insertion")]
      $item
    )*
  };
}

macro_rules! cfg_feat_mutation {
	($($item:item)*) => {
    $(
      #[cfg(any(feature = "destroy", feature = "insertion"))]
      $item
    )*
  };
}
