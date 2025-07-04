export class Location {
  name: string
  address: string
  city: string
  province: string
  series: Array<number>
  phone: string

  constructor(
    name: string,
    address: string,
    city: string,
    province: string,
    series: Array<number>,
    phone: string
  ) {
    this.name = name
    this.address = address
    this.city = city
    this.province = province
    this.series = series
    this.phone = phone
  }
}
