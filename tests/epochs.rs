use {super::*, ord::subcommand::epochs::Output, ord::Sat};

#[test]
fn empty() {
  assert_eq!(
    CommandBuilder::new("epochs").output::<Output>(),
    Output {
      starting_sats: vec![
        Sat(0),
        Sat(4200000000000000),
        Sat(6300000000000000),
        Sat(7350000000000000),
        Sat(7875000000000000),
        Sat(8137500000000000),
        Sat(8268750000000000),
        Sat(8334375000000000),
        Sat(8367187500000000),
        Sat(8383593750000000),
        Sat(8391796875000000),
        Sat(8395898437080000),
        Sat(8397949218120000),
        Sat(8398974608640000),
        Sat(8399487303480000),
        Sat(8399743650480000),
        Sat(8399871823560000),
        Sat(8399935909680000),
        Sat(8399967952320000),
        Sat(8399983973640000),
        Sat(8399991983880000),
        Sat(8399995989000000),
        Sat(8399997991560000),
        Sat(8399998992840000),
        Sat(8399999493480000),
        Sat(8399999743800000),
        Sat(8399999868960000),
        Sat(8399999931120000),
        Sat(8399999962200000),
        Sat(8399999977320000),
        Sat(8399999984880000),
        Sat(8399999988240000),
        Sat(8399999989920000),
        Sat(8399999990760000)
      ]
    }
  );
}