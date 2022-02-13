macro_rules! make_text_endpoints {
    (
        $ i: tt
        || $ _: tt
    ) => {
        $ i
    };
    (|| $ i: tt) => {
        $ i
    };
    (
        $ i: tt
        ||= $ _: tt >=>
        $ cb: ident |>
        $ (
            $ arg: tt
        ) *
    ) => {
        $ cb! {
            $ (
                $ arg
            ) *
            $ i
        }
    };
    (
        ||= $ i: tt >=>
        $ cb: ident |>
        $ (
            $ arg: tt
        ) *
    ) => {
        $ cb! {
            $ (
                $ arg
            ) *
            $ i
        }
    };
    (
        |-> $ endpoint :ident
        ->> $ field :ident
    ) => {
        paste::paste! {
            #[derive(
                serde::Deserialize
            )] pub struct [<
                $ endpoint
                Model
            >] {
                $ field: $ crate::UrlString,
            }
        }
    };
    (
        $ (
            $ (
                #[
                    $ attr :meta
                ]
            ) *
            $ endpoint :ident
            $ (
                @ $ l :lifetime
            ) ?
            $ (
                |>
                    $ (
                        $ inner_type: ty
                    ) ?
                    :=
                    $ (
                        $ inner :ident
                    ) ?
                <|
            ) ?
            $ (
                ~~> $ ep_alias :ident
            ) ?
            $ (
                !# $range :expr
            ) ?
        ) => +
    ) => {
        $ (
            #[derive(
                Debug,
                PartialEq,
                Eq,
                Clone,
                Copy,
            )] $ (
                #[
                    $ attr
                ]
            ) *
            pub struct $ endpoint $ (
                <
                    $ l
                >
            ) ? $ (
                (
                    pub & $ l
                    $ (
                        $ inner_type
                    ) ?
                )
            ) ? ;

            paste::paste! {
                make_text_endpoints! {
                    $ (
                        $ (
                            $ inner
                        ) ?
                    ) ?
                        ||=
                    [<
                        $ endpoint :lower
                    >]
                        >=>
                    make_text_endpoints |>
                        |-> $ endpoint
                        ->>
                }
            }

            impl $ (
                <
                    $ l
                >
            ) ? $ crate
                ::implementation
                ::types
                ::IntoUrl
            for $ endpoint $ (
                <
                    $ l
                >
            ) ? {
                type Response = $ crate::types::UrlString;

                type Fut = into_url_fut! {};

                fn into_url(
                    self,
                ) -> $ crate::types::Result<
                    url::Url
                > {
                    paste::paste! {
                        Ok({
                            #[allow(
                                unused_mut
                            )] let mut url: url::Url =
                                $ crate
                                    ::r#static
                                    ::BASEURL
                                        .join(
                                            make_text_endpoints! {
                                                $ (
                                                    $ ep_alias
                                                ) ?
                                                    ||=
                                                [<
                                                    $ endpoint :lower
                                                >]
                                                    >=>
                                                stringify |>
                                            }
                                        ) ? ;
                            $ (
                                $ (
                                    url
                                        .query_pairs_mut()
                                        .append_pair(
                                            "text",
                                            if matches! {
                                                (
                                                    self.0 as
                                                    &$ l $ inner_type
                                                )
                                                    .len(),
                                                $ range
                                            } { self.0 } else {
                                                Err($crate
                                                    ::NekosLifeError
                                                    ::OutOfRangeError {
                                                    endpoint_name: stringify! {
                                                        $ endpoint
                                                    }.to_owned(),
                                                    range: $ range,
                                                })?
                                            }
                                        ) ;
                                ) ?
                            ) ?

                            url
                        })
                    }
                }

                paste::paste! {
                    make_text_endpoints! {
                        $ (
                            $ (
                                $ inner
                            ) ?
                        ) ?
                            ||=
                        [<
                            $ endpoint :lower
                        >]
                            >=>
                        parse_json |>
                            [<
                                $ endpoint
                                Model
                            >]
                            ,
                    }
                }
            }
        ) +
    };
}